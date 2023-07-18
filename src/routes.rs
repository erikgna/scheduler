use rocket::serde::json::Json;
use crate::models::User;
use crate::user_models::{UserLogin, UserToken};

// #[post("/newUser", format = "application/json", data = "<new_user>")]
// pub fn new_user(new_user: Json<NewUser>) -> JsonValue {
//     let result = match User::insert_user(new_user.into_inner()) {
//         Ok(_) => {
//             json!({
//                 "status": 200,
//                 "result": "User created",
//             })
//         }
//         Err(error) => {
//             json!({
//                 "status": 500,
//                 "error": format!("Error creating user: {}", error),
//             })
//         }
//     };

//     JsonValue(result)
// }

#[post("/login", data = "<user_login>")]
pub fn login(user_login: Json<UserLogin>) -> Json<UserToken> {
        let token =  User::login(user_login.into_inner()).unwrap();    
        Json(UserToken{token: token})
        // Ok(token) => {
        //     json!({
        //         "status": 200,
        //         "result": "Login successful",
        //         "token": token,
        //     })
        // }        
        // Err(error) => {
        //     json!({
        //         "status": 500,
        //         "error": format!("Error logging in: {}", error),
        //     })
        // }    
}

#[get("/teste")]
pub fn teste() -> &'static str {
    "Hello there! I'm a string!"
}