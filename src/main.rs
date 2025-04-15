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
    let config = crate::functions::utility::toml::fromFile("../../config.toml");

    /////////////////////////////////
    
    CorePlugin::initialize(config)
        .expect("ERROR: failed to initialize CorePlugin");

    /////////////////////////////////

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

    BackendAppFramework.start(listener)
        .unwrap()
            .join()
                .unwrap();
}
