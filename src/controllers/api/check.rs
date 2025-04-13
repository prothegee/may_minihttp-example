pub mod status {
    use yarte::Serialize;
    use may_minihttp::{Request, Response};

    use crate::{constants::{content, http}, types::status::TStatusConfig};

    #[allow(non_snake_case, unused_variables)]
    pub fn config(req: Request, resp: &mut Response) {
        resp.header(content::content_http_type::APPLICATION_JSON);

        match req.method() {
            http::method::GET => {
                let version = env!("CARGO_PKG_VERSION");

                let config = crate::functions::utility::toml::fromFile("../backend/rust/com_prothegee_api/config.toml");

                let status = config["config"]["status"].as_integer().expect("config status is not right") as i8;
                let release_date = config["config"]["release_date"].as_str().expect("config release_date is not right");

                let callbackData = TStatusConfig {
                    status: status,
                    version: version,
                    release_date: release_date
                };

                callbackData.to_bytes_mut(resp.body_mut());
            }
            _ => {
                resp.status_code(405, "not allowed");
            }
        }
    }
}