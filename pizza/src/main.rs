#![feature(proc_macro_hygiene, decl_macro)]

macro_rules! hashmap {
    ($( $key: expr => $val: expr ),*) => {{
         let mut map = ::std::collections::HashMap::new();
         $( map.insert($key, $val); )*
         map
    }}
}

#[macro_use] extern crate rocket;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate diesel;
#[macro_use] extern crate lazy_static;

use std::collections::HashMap;

use rocket_contrib::json::Json;
use serde_json::{Value, json};

mod pizza;
use pizza::{Pizza, NewPizza, IgnoreNoneFieldsUpdatePizza};

pub mod schema;
pub mod db;

lazy_static! {
    static ref PIZZA_PRICING: HashMap<String, f64> = hashmap!["Hawaiian".to_string() => 9.95, "Veggie".to_string() => 7.95, "Pepperoni".to_string() => 11.95];
}

#[post("/", data = "<pizza>")]
fn create(pizza: Json<IgnoreNoneFieldsUpdatePizza>, connection: db::Connection) -> Json<Value> {
    let pizza_name = pizza.name.to_string();
    match PIZZA_PRICING.get(&pizza_name) {
        Some(_) => {
            let insert = NewPizza { name: &pizza_name, price: PIZZA_PRICING[&pizza_name] };
            Pizza::create(insert, &connection);
            Json(json!(format!("{{ \"message\": \"{} pizza successfully added. \"}}", pizza_name)))
        },
        None => Json(json!({ "failure": "Didn't work" })),
    }
}

#[get("/list")]
fn read(connection: db::Connection) -> Json<Value> {
    Json(json!(Pizza::read(&connection)))
}

#[put("/<id>", data = "<pizza>")]
fn update(id: i32, pizza: Json<IgnoreNoneFieldsUpdatePizza>, connection: db::Connection) -> Json<Value> {
    let pizza_name = pizza.name.to_string();
    match PIZZA_PRICING.get(&pizza_name) {
        Some(_) => {
            let update = IgnoreNoneFieldsUpdatePizza { name: &pizza_name };
            Json(json!({
                "success": Pizza::update(id, update, &connection)
            }))
        },
        None => Json(json!({ "failure": "Didn't work" }))
    }
}

#[delete("/<id>")]
fn delete(id: i32, connection: db::Connection) -> Json<Value> {
    Json(json!({
        "success": Pizza::delete(id, &connection)
    }))
}

fn main() {
    rocket::ignite()
        .mount("/pizza", routes![read, create, update, delete])
        .manage(db::connect())
        .launch();
}