use backend::BackendAppFramework;
use may_minihttp::HttpServiceFactory;
use plugins::{core::CorePlugin, traits::TraitPlugin};

mod backend;
mod constants;
mod controllers;
mod functions;
mod plugins;
mod route;
mod types;

fn main() {
    let config = crate::functions::utility::toml::from_file("../../config.toml");

    // 6 Mb stack size
    may::config().set_stack_size(6 * 1024 * 1024);

    /////////////////////////////////
    
    CorePlugin::initialize(config)
        .expect("ERROR: failed to initialize CorePlugin");

    /////////////////////////////////

    let app = BackendAppFramework::new(&config);

    let listeners = config["config"]["listeners"]
        .as_array()
        .expect("listeners should be an array object that has address & port key");

    let first = &listeners[0];

    let listener = format!("{}:{}",
        first["address"].as_str().unwrap(),
        first["port"].as_integer().unwrap()
    );

    /////////////////////////////////

    println!("INFO: starting backend {}", listener);

    app.start(listener)
        .unwrap()
            .join()
                .unwrap();
}