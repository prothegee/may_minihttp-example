use std::{collections::HashMap, fs, io, path::PathBuf};
use may_minihttp::{Request, Response};
use regex::Regex;

pub type RouteHandler = fn(Request, &mut Response);

#[allow(dead_code)]
struct Route {
    pattern: Regex,
    param_names: Vec<String>,
    handler: RouteHandler
}

pub struct Router {
    routes: Vec<Route>,
    static_routes: HashMap<String, RouteHandler>,
    public_dir: PathBuf
}
impl Router {
    fn parse_path_pattern(path: &str) -> (Regex, Vec<String>) {
        let mut param_names = Vec::new();
        let pattern_str = path
            .split('/')
            .map(|part| {
                if part.starts_with('{') && part.ends_with('}') {
                    let param_name = part[1..part.len() - 1].to_string();
                    param_names.push(param_name);
                    "([^/]+)"
                } else {
                    part
                }
            })
            .collect::<Vec<_>>()
            .join("/");

        let full_pattern = format!("^{}$", pattern_str);

        (Regex::new(&full_pattern).unwrap(), param_names)
    }

    ////////////////////////////////////

    pub fn with_static(public_dir: PathBuf) -> Self {
        Router {
            routes: Vec::new(),
            static_routes: HashMap::new(),
            public_dir,
        }
    }

    pub fn add_route(&mut self, path: &str, handler: RouteHandler) {
        if path.contains('{') {
            let (pattern, param_names) = Self::parse_path_pattern(path);
            self.routes.push(Route {
                pattern,
                param_names,
                handler,
            });
        } else {
            self.static_routes.insert(path.to_string(), handler);
        }
    }

    fn try_serve_static(&self, req: &Request, resp: &mut Response) -> io::Result<bool> {
        // strip query string
        let path = req.path().splitn(2, '?').next().unwrap_or("/");
        let rel = path.trim_start_matches('/');
        
        // pick root directory canonicalize to stop ../ attacks
        let root = match self.public_dir.canonicalize() {
            Ok(p) => p,
            Err(_) => return Ok(false),
        };
        let file_path = root.join(rel);
        let abs = match file_path.canonicalize() {
            Ok(p) if p.starts_with(&root) => p,
            _ => return Ok(false),
        };
        if !abs.is_file() {
            return Ok(false);
        }

        // read file to memory
        let body = fs::read(&abs)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("read error: {}", e)))?;

        // nothing there
        if body.is_empty() {
            return Ok(false);
        }

        // mime check
        let mime = mime_guess::from_path(&abs).first_or_octet_stream();

        // set header content type & apply the body content
        resp.header(format!("Content-Type: {}", mime))
            .body_vec(body);

        Ok(true)
    }

    pub fn handle_request(&self, req: Request, resp: &mut Response) {
        let base = req.path().splitn(2, '?').next().unwrap_or("");

        if let Ok(true) = self.try_serve_static(&req, resp) {
            return;
        }

        if let Some(h) = self.static_routes.get(base) {
            h(req, resp);
            return;
        }

        for route in &self.routes {
            if route.pattern.is_match(base) {
                (route.handler)(req, resp);
                return;
            }
        }

        resp.status_code(404, "Not Found")
            .header("Content-Type: text/plain")
            .body("404 Not Found");
    }
}