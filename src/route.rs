use std::collections::HashMap;
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
    static_routes: HashMap<String, RouteHandler>
}
#[allow(dead_code)]
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

    pub fn new() -> Self {
        Router {
            routes: Vec::new(),
            static_routes: HashMap::new()
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

    pub fn handle_request(&self, req: Request, resp: &mut Response) {
        let path = req.path();

        if let Some(handler) = self.static_routes.get(path) {
            handler(req, resp);
            return;
        }

        for route in &self.routes {
            if let Some(_captures) = route.pattern.captures(path) {
                // if you need to use parameters, you can access them via captures
                // e.g.: captures.get(1).map(|m| m.as_str())
                (route.handler)(req, resp);
                return;
            }
        }

        resp.status_code(404, "not found, what are you doing?");
    }
}