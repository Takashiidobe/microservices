use rocket_contrib::json::Json;
use serde_json::{Value, json};
use crate::{NewApiKey, ApiKey};
use crate::db;

#[get("/list")]
pub fn read_api_keys(connection: db::Connection) -> Json<Value> {
    Json(json!(ApiKey::read_all(&connection)))
}

#[post("/", data="<data>")]
pub fn create_api_key(data: Json<NewApiKey>, connection: db::Connection) -> Json<Value> {
    let public_id = data.public_id;
    let permission;
    match data.permission {
        "ADMIN" => permission = "ADMIN",
        "USER" => permission = "USER",
        _ => return Json(json!({ "failure": "ApiKey was not created" }))
    }
    let result = ApiKey::create(NewApiKey { public_id, permission }, &connection);
    match result {
        true => Json(json!({ "success": "ApiKey was created" })),
        _ => Json(json!({ "failure": "ApiKey was not created" }))
    }
}
