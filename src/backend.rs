use std::{io, fs, path::PathBuf};
use may_minihttp::{HttpService, HttpServiceFactory, Request, Response};
use crate::route::Router;

pub struct BackendRouter {
    router: Router,
    public_dir: PathBuf,
    upload_dir: PathBuf,
}

impl BackendRouter {
    pub fn new(public_dir: PathBuf, upload_dir: PathBuf) -> Self {
        let mut r: Router = Router::new();

        // /api/test*
        {
            r.add_route("/api/test/hello", 
            crate::controllers::api::test::hello);
        }

        // /api/check*
        {
            r.add_route("/api/check/status/config",
                crate::controllers::api::check::status::config);
        }

        // /api/param*
        {
            r.add_route("/api/param", 
                crate::controllers::api::param::message);
        }

        // /api/path*
        {
            r.add_route("/api/path/{path1}/{path2}",
                crate::controllers::api::path::handle_path);
        }

        // /api/submit*
        {
            r.add_route("/api/submit/item", 
                crate::controllers::api::submit::item);
        }

        BackendRouter {
            router: r,
            public_dir,
            upload_dir,
        }
    }

    // try to serve a static file; returns true if we wrote a response.
    fn try_serve_static(&self, req: &Request, resp: &mut Response) -> io::Result<bool> {
        // strip query string
        let path = req.path().splitn(2, '?').next().unwrap_or("/");

        // choose which root to serve from:
        let root = if path.starts_with("/upload/") {
            &self.upload_dir
        } else {
            &self.public_dir
        };

        // sanitize leading slash
        let rel = &path.trim_start_matches('/');
        let file_path = root.join(rel);

        if file_path.is_file() {
            // read the file into memory
            let body = fs::read(&file_path)
                .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("read error: {}", e)))?;

            // guess mime type
            let mime = mime_guess::from_path(&file_path).first_or_octet_stream();
            let header_value = format!("Content-Type: {}", mime.as_ref());

            /*
            WARNING: may leak, don't know how much
            NOTE:
            - leak change:
                - move the static &str to Vector?
            */
            let header_static: &'static str = Box::leak(header_value.into_boxed_str());
            resp
                .header(header_static)
                .body_vec(body);
            return Ok(true);
        }
        Ok(false)
    }
}

impl HttpService for BackendRouter {
    fn call(&mut self, req: Request, resp: &mut Response) -> io::Result<()> {
        if self.try_serve_static(&req, resp)? {
            return Ok(());
        }
        self.router.handle_request(req, resp);
        Ok(())
    }
}

pub struct BackendAppFramework {
    public_dir: PathBuf,
    upload_dir: PathBuf
}
impl BackendAppFramework {
    pub fn new(config: &toml::Value) -> Self {
        let base = config["config"]["dir_public_path"]
            .as_str().unwrap();
        let up   = config["config"]["dir_public_path_upload"]
            .as_str().unwrap();
        BackendAppFramework {
            public_dir: PathBuf::from(base),
            upload_dir: PathBuf::from(up),
        }
    }
}

impl HttpServiceFactory for BackendAppFramework {
    type Service = BackendRouter;

    fn new_service(&self, _id: usize) -> Self::Service {
        BackendRouter::new(self.public_dir.clone(), self.upload_dir.clone())
    }
}