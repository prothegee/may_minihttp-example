use std::io;
use may_minihttp::{HttpService, HttpServiceFactory, Request, Response};
use crate::route::Router;

pub struct BackendRoutes {
    router: Router,
}

impl BackendRoutes {
    pub fn new() -> Self {
        let mut router = Router::new();

        router.add_route("/api/test/hello", crate::controllers::api::test::hello);

        BackendRoutes { router }
    }
}

impl HttpService for BackendRoutes {
    fn call(&mut self, req: Request, resp: &mut Response) -> io::Result<()> {
        self.router.handle_request(req, resp);
        Ok(())
    }
}

pub struct BackendAppFramework;

impl HttpServiceFactory for BackendAppFramework {
    type Service = BackendRoutes;

    fn new_service(&self, _id: usize) -> Self::Service {
        BackendRoutes::new()
    }
}