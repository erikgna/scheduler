use super::db::Conn as DbConn;
use rocket_contrib::json::{Json, JsonValue};
use super::models::{User, NewUser, UserData};
use serde_json::Value;

#[post("/newUser", format = "application/json", data = "<new_user>")]
pub fn new_user(conn: DbConn, new_user: Json<NewUser>) -> Json<Value> {
    Json(json!({
        "status": User::insert_user(new_user.into_inner(), &conn),
        "result": "User created",
    }))
}

#[post("/login", format = "application/json", data = "<user_data>")]
pub fn login(conn: DbConn, user_data: Json<UserData>) -> JsonValue {
    let result = match User::login(user_data.into_inner(), &conn) {
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