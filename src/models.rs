use diesel;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use rand::{Rng, distributions::Alphanumeric};
use argon2::{Config, hash_encoded, verify_encoded};
use super::enums::{LoginResult};
use super::schema::users;
// this is to get users from the database
#[derive(Serialize, Queryable)] 
pub struct User {
    pub id: i32,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub password: String,
    pub token: Option<String>,
}

// decode request data
#[derive(Deserialize)] 
pub struct UserData {
    pub email: String,
    pub password: String,
}
// this is to insert users to database
#[derive(Serialize, Deserialize, Insertable)]
#[table_name = "users"]
pub struct NewUser {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,    
}

impl User {
    pub fn insert_user(mut user: NewUser, conn: &PgConnection) -> Result<(), diesel::result::Error> {
        let pass = User::hash_password(&user.password);
        user.password = pass.unwrap();

        match diesel::insert_into(users::table)
            .values(&user)
            .execute(conn)
        {
            Ok(_) => Ok(()),
            Err(error) => {
                println!("Error inserting user: {:?}", error);
                Err(error)
            }
        }
    }

    pub fn verify_password(password: &str, password_encoded: &[u8]) -> bool {
        print!("password: {:?}, password_encoded: {:?}", password, password_encoded);
        if let Ok(result) = verify_encoded(password, password_encoded) {
            result
        } else {
            false
        }
    }
    
    pub fn login(auth: UserData, conn: &PgConnection) -> Result<LoginResult, diesel::result::Error> {
        use crate::schema::users::dsl::*;
    
        if let Some(user) = users.filter(email.eq(&auth.email)).first::<User>(conn).optional()? {
            if User::verify_password(&auth.password, user.password.as_bytes()) {
                Ok(LoginResult::Success)
            } else {
                Ok(LoginResult::IncorrectPassword)
            }
        } else {
            Ok(LoginResult::UserNotFound)
        }
    }
    
    pub fn hash_password(password: &str) -> Result<String, argon2::Error> {
        let salt: String = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(32)
            .map(char::from)
            .collect();
    
        let config = Config::default();
    
        hash_encoded(password.as_bytes(), salt.as_bytes(), &config)
    }
}