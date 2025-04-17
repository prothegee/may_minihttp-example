pub mod status {
    use yarte::Serialize;
    use may_minihttp::{Request, Response};

    use crate::{constants::http, types::status::TStatusConfig};

    pub fn config(req: Request, resp: &mut Response) {
        resp.header(http::content_type::CT_APPLICATION_JSON);

        // maybe the's a condition where you want to use multiple methods in single endpoint
        match req.method() {
            http::method::GET => {
                let version = env!("CARGO_PKG_VERSION");

                let config = crate::functions::utility::toml::from_file("../backend/rust/com_prothegee_api/config.toml");

                let status = config["config"]["status"].as_integer().expect("config status is not right") as i8;
                let release_date = config["config"]["release_date"].as_str().expect("config release_date is not right");

                let callback = TStatusConfig {
                    status: status,
                    version: version,
                    release_date: release_date
                };

                callback.to_bytes_mut(resp.body_mut());
            }
            _ => {
                resp.status_code(405, "not allowed");
            }
        }
    }
}