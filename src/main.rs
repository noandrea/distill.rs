#![allow(redundant_semicolons)]
#![feature(plugin)]
#![feature(slice_concat_ext)]
#![feature(proc_macro_hygiene, decl_macro)]
#![feature(try_trait)]
extern crate backtrace;
extern crate blake3;
extern crate clap;
extern crate dotenv;
extern crate env_logger;
extern crate flexi_logger;
extern crate futures;
extern crate hex;
extern crate itertools;
#[macro_use]
extern crate log;
extern crate log4rs;
#[macro_use]
extern crate rocket;
extern crate rocket_contrib;
extern crate rocket_cors;
extern crate sled;
extern crate sled_extensions;


use std::thread;

use clap::clap_app;
use log::LevelFilter;
use log4rs::config::{Appender, Config, Root};
use std::env;

pub mod server;
pub mod utils;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");


fn init_logging() {
    match env::var("LOG_CONF") {
        Ok(x) => {
            let mut deserializers = log4rs::file::Deserializers::default();
            let _result = log4rs::init_file(x, deserializers).unwrap();
        }
        Err(_) => {
            let stdout = log4rs::append::console::ConsoleAppender::builder().build();
            let config = Config::builder()
                .appender(Appender::builder().build("console", Box::new(stdout)))
                .build(Root::builder().appender("console").build(LevelFilter::Warn))
                .unwrap();
            let _handle = log4rs::init_config(config).unwrap();
        }
    }
}

fn main() {
    dotenv::dotenv().ok();

    init_logging();

    let matches = clap_app!(mdw =>
        (name: env!("CARGO_PKG_NAME"))
        (version: VERSION)
        (author: env!("CARGO_PKG_AUTHORS"))
        (about: env!("CARGO_PKG_DESCRIPTION"))
            (@arg debug: --debug "Enable debug logging")
            (@arg CONFIG: -c --config +takes_value "Use this configuration file instead of the default one")
            (@subcommand start =>
                (name: "start")
                (about: "Start Distill")
            )
            (@subcommand import =>
                (name: "import")
                (about: "import data from json / csv files")
                (@arg INPUT: +required "Import data from a csv file")
            )
            (@subcommand export =>
                (name: "export")
                (about: "Export data to json or csv")
                (@arg export_format: --format "format toe export data to")
                (@arg out_file: -o --output +takes_value "Save the data to file instead of console")
            )
    ).get_matches();
    //let url = env::var("NODE_URL")
     //   .expect("NODE_URL must be set")
      //  .to_string();


    if let Some(v_matches) = matches.subcommand_matches("start") {
        debug!("Starting Distill");
        // this should start the web server
        server::start();
        loop {
            // just to stop main() thread exiting.
            thread::sleep(std::time::Duration::new(40, 0));
        }
    }
}
