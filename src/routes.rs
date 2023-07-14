use super::db::Conn as DbConn;
use rocket_contrib::json::{Json, JsonValue};
use super::models::{User, NewUser, UserData};

#[post("/newUser", format = "application/json", data = "<new_user>")]
pub fn new_user(conn: DbConn, new_user: Json<NewUser>) -> JsonValue {
    let result = match User::insert_user(new_user.into_inner(), &conn) {
        Ok(_) => {
            json!({
                "status": 200,
                "result": "User created",
            })
        }
        Err(error) => {
            json!({
                "status": 500,
                "error": format!("Error creating user: {}", error),
            })
        }
    };

    JsonValue(result)
}

#[post("/login", format = "application/json", data = "<user_data>")]
pub fn login(conn: DbConn, user_data: Json<UserData>) -> JsonValue {
    let result = match User::login(user_data.into_inner(), &conn) {
        Ok(token) => {
            json!({
                "status": 200,
                "result": "Login successful",
                "token": token,
            })
        }        
        Err(error) => {
            json!({
                "status": 500,
                "error": format!("Error logging in: {}", error),
            })
        }
    };

    JsonValue(result)
}

#[get("/teste", format = "application/json")]
pub fn teste() -> JsonValue {
    JsonValue( json!({
        "status": 200,
        "result": "Login successful",        
    }))
}