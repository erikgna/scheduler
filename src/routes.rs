use super::db::Conn as DbConn;
use rocket_contrib::json::{Json, JsonValue};
use super::models::{User, NewUser};
use serde_json::Value;

#[post("/newUser", format = "application/json", data = "<new_user>")]
pub fn new_user(conn: DbConn, new_user: Json<NewUser>) -> Json<Value> {
    Json(json!({
        "status": User::insert_user(new_user.into_inner(), &conn),
        "result": "User created",
    }))
}

#[post("/getUser", format = "application/json", data = "<email>")]
pub fn find_user(conn: DbConn, email: Json<String>) -> JsonValue {
    let result = match User::get_user_by_email(&email, &conn) {
        Ok(user) => {
            json!({
                "status": 200,
                "result": user,
            })
        }
        Err(_) => {
            json!({
                "status": 404,
                "message": "User not found",
            })
        }
    };

    JsonValue(result)
}