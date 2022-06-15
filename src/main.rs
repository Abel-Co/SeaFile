#![allow(warnings)]

#[macro_use] extern crate diesel;
#[macro_use] extern crate serde_derive;
extern crate serde;
extern crate serde_json;
extern crate futures;
extern crate num_cpus;
extern crate actix_web;
extern crate env_logger;
extern crate dotenv;
extern crate chrono;
extern crate bcrypt;
extern crate regex;
extern crate http;
extern crate postgres;
extern crate timeago;
extern crate pulldown_cmark;
extern crate openssl;
extern crate jsonwebtoken as jwt;
extern crate md5;
extern crate ring;

use actix_web::{server,actix::System};
use crate::model::db::init;
// use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};

mod api;
mod handler;
mod model;
mod share;
mod utils;
mod router;

fn main() {
    ::std::env::set_var("RUST_LOG", "rustlang-cn=info");
    ::std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();
    let sys = System::new("rustlang-cn");
    let addr = init();

    // let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    // builder.set_private_key_file("privkey.pem", SslFiletype::PEM).unwrap();
    // builder.set_certificate_chain_file("fullchain.pem").unwrap();

    server::new( move || router::app_state(addr.clone()))
         .bind("localhost:8000").unwrap()
         .shutdown_timeout(2)
         .start();

    sys.run();
}

