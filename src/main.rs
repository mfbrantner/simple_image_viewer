#![deny(unused_results)]
#![deny(unused_variables)]
#![deny(warnings)]
mod config;
mod application;
mod args;
mod folder;
use config::Config;
use args::Args;
use application::Application;

fn main() {


    let config = match Args::parse().and_then(Config::from) {
        Ok(c) => c,
        Err(e) => { eprintln!("Could not create run config: {}", e); return; }
    };

    let app = match Application::from(config) {
        Ok(a) => a,
        Err(e) => {eprintln!("Error during application startup: {}", e); return;}
    };

    match app.run() {
        Ok(()) => (),
        Err(e) => {eprintln!("Application stopped with error: {}", e);}
    }

}

