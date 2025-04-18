use std::{io, path::PathBuf};
use may_minihttp::{HttpService, HttpServiceFactory, Request, Response};
use crate::route::Router;

pub struct BackendRouter {
    router: Router,
}

impl BackendRouter {
    pub fn new(public_dir: PathBuf) -> Self {
    // pub fn new(public_dir: PathBuf, _upload_dir: PathBuf) -> Self {
        // let mut router: Router = Router::new();
        let mut router: Router = Router::with_static(public_dir);

        // /api/test*
        {
            router.add_route("/api/test/hello", 
            crate::controllers::api::test::hello);
        }

        // /api/check*
        {
            router.add_route("/api/check/status/config",
                crate::controllers::api::check::status::config);
        }

        // /api/param*
        {
            router.add_route("/api/param", 
                crate::controllers::api::param::message);
        }

        // /api/path*
        {
            router.add_route("/api/path/{path1}/{path2}",
                crate::controllers::api::path::handle_path);
        }

        // /api/submit*
        {
            router.add_route("/api/submit/item", 
                crate::controllers::api::submit::item);
        }

        BackendRouter { router }
    }
}

impl HttpService for BackendRouter {
    fn call(&mut self, req: Request, resp: &mut Response) -> io::Result<()> {
        self.router.handle_request(req, resp);
        Ok(())
    }
}

pub struct BackendAppFramework {
    public_dir: PathBuf
}
impl BackendAppFramework {
    pub fn new(config: &toml::Value) -> Self {
        let base = config["config"]["dir_public_path"]
            .as_str().unwrap();
        BackendAppFramework {
            public_dir: PathBuf::from(base)
        }
    }
}

impl HttpServiceFactory for BackendAppFramework {
    type Service = BackendRouter;

    fn new_service(&self, _id: usize) -> Self::Service {
        BackendRouter::new(self.public_dir.clone())
    }
}