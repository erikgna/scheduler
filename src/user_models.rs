use rocket::serde::Deserialize;

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct UserLogin {
    email: String,
    password: String,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct UserToken {
    token: String
}