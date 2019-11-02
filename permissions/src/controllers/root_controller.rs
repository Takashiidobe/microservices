use rocket_contrib::json::Json;
use serde_json::{Value, json};

#[get("/")]
pub fn root() -> Json<Value> {
    Json(json!({ "message": "The server is alive" }))
}