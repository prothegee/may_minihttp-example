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