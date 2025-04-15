use std::io;
use std::sync::Once;
use crate::plugins::{core::CorePlugin, traits::TraitPlugin};
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
        // plugins initialized once per worker
        // might be doesn't exists for some backend when it start
        static INIT: Once = Once::new();
        INIT.call_once(|| {
            let config = crate::functions::utility::toml::fromFile("../../config.toml");
            CorePlugin::initialize(config).expect("Plugin init failed");
        });
        
        BackendRouter::new()
    }
}