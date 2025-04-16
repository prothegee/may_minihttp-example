use std::io;
use may_minihttp::{HttpService, HttpServiceFactory, Request, Response};
use crate::route::Router;

pub struct BackendRouter {
    router: Router,
}

impl BackendRouter {
    pub fn new() -> Self {
        let mut router = Router::new();

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
            // maybe: /api/path/{path1}/{path2}
            // check those and get path1 & path2 value on the controleer
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

pub struct BackendAppFramework;

impl HttpServiceFactory for BackendAppFramework {
    type Service = BackendRouter;

    fn new_service(&self, _id: usize) -> Self::Service {
        BackendRouter::new()
    }
}