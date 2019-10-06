use rocket_contrib::json::Json;
use serde_json::{Value, json};
use bcrypt::{hash, BcryptError, verify};
use crate::user::*;
use crate::db;

#[get("/list")]
pub fn read(connection: db::Connection) -> Json<Value> {
    Json(json!(User::read_all(&connection)))
}

#[post("/", data="<data>")]
pub fn create(data: Json<NewUser>, connection: db::Connection) -> Result<Json<Value>, BcryptError> {
    let username = data.username;
    let password = &hash(data.password, 6)?;
    User::create(    NewUser { username, password }, &connection);
    Ok(Json(json!({ "success": "User was created" })))
}

#[put("/<id>", data="<data>")]
pub fn update(id: i32, data: Json<UpdateUser>, connection: db::Connection) -> Result<Json<Value>, BcryptError> {
    let hashed = &hash(data.password, 6)?;
    let updated_user = UpdateUser { password: hashed };
    User::update(id, updated_user, &connection);
    Ok(Json(json!({ "success": "User was updated" })))
}

#[delete("/<id>")]
pub fn delete(id: i32, connection: db::Connection) -> Json<Value> {
    User::delete(id, &connection);
    Json(json!({ "success": "User was deleted" }))
}

#[get("/<id>")]
pub fn get(id: i32, connection: db::Connection) -> Json<Value> {
    let user = User::read(id, &connection);
    Json(json!({ "success": user }))
}

#[post("/<id>", data="<data>")]
pub fn sign_in(id: i32, data: Json<UpdateUser>, connection: db::Connection) -> Json<Value> {
    let user = User::read(id, &connection);
    let result = verify(data.password, &user.password);
    match result {
        Ok(true) => Json(json!({ "success": "succeeded" })),
        _ => Json(json!({ "failure": "didn't work" }))
    }
}