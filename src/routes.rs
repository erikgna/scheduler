use crate::middleware::AuthorizedUser;
use crate::models::User;
use crate::user_models::{UserLogin, UserToken, NewUser};
use rocket::serde::json::Json;
use rocket::http::Status;
use rocket::response::{status::Created, status::Custom, status};

#[post("/register", format = "application/json", data = "<new_user>")]
pub fn register(new_user: Json<NewUser>) -> Result<Created<Json<&'static str>>, Custom<&'static str>> {
    match User::insert_user(new_user.into_inner()) {
        Ok(_) => Ok(Created::new("/user/1").body(Json("User registered successfully!"))),
        Err(_) => Err(Custom(Status::InternalServerError, "Failed to register user.")),
    }
}

#[post("/login", data = "<user_login>")]
pub fn login(user_login: Json<UserLogin>) -> Result<status::Custom<Json<UserToken>>, status::Custom<&'static str>> {
    match User::login(user_login.into_inner()) {
        Ok(token) => {
            let response = UserToken { token };
            Ok(status::Custom(Status::Ok, Json(response)))
        }
        Err(e) => {
            match e {
                diesel::result::Error::NotFound => {
                    Err(Custom(Status::NotFound, "User not found"))
                }
                _ => Err(Custom(Status::InternalServerError, "Failed to log in")),
            }
        }
    }
}

#[get("/teste")]
pub fn teste(auth: AuthorizedUser) -> &'static str {
    print!("User ID: {}", auth.user_id);
    "Hello there! I'm a string!"
}