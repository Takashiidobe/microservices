#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate diesel;

mod models;
use models::{db, schema};

mod controllers;
use controllers::root_controller::*;
use controllers::api_key_controller::*;
use controllers::user_controller::*;

mod records;
use records::api_key::*;
use records::user::*;

fn main() {
    rocket::ignite()
        .mount("/users",
               routes![
                root,
                get,
                read,
                update,
                sign_in,
                delete,
                create
             ]
        )
        .mount(
            "/api_keys",
            routes![
                read_api_keys,
                create_api_key
            ]
        )
        .manage(db::connect())
        .launch();
}