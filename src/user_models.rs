use rocket::serde::{Deserialize, Serialize};
use diesel::Insertable;
use super::schema::users;

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct UserLogin {
    pub email: String,
    pub password: String,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct UserToken {
    pub token: String
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct NewUser {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,    
}

#[derive(Insertable, PartialEq, Eq, Debug, Clone)]
#[table_name = "users"]
pub struct NewUserInsert {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
}

// Implement a conversion function from `NewUser` to `NewUserInsert`
impl From<NewUser> for NewUserInsert {
    fn from(user: NewUser) -> Self {
        NewUserInsert {
            first_name: user.first_name,
            last_name: user.last_name,
            email: user.email,
            password: user.password,
        }
    }
}