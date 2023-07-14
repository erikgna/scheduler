use diesel;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use argon2::{Config, hash_encoded};
use rand::Rng;
use rand::distributions::Alphanumeric;
use super::enums::LoginResult;
use super::schema::users;
use jsonwebtoken::{encode, Header, EncodingKey};
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
#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    id: i32,        // ID do usuário
    email: String,  // Email do usuário
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
        argon2::verify_encoded(password, password_encoded).unwrap()
    }
    
    pub fn login(auth: UserData, conn: &PgConnection) -> Result<String, diesel::result::Error> {
        use crate::schema::users::dsl::*;
    
        if let Some(user) = users.filter(email.eq(&auth.email)).first::<User>(conn).optional()? {
            if User::verify_password(&user.password, auth.password.as_bytes()) {
                // Gerar um token JWT com o ID e o email do usuário
                let token_data = Claims {
                    id: user.id,
                    email: user.email.clone(),
                };
    
                let to_send_token = encode(
                    &Header::default(),
                    &token_data,
                    &EncodingKey::from_secret("sua_chave_secreta".as_ref()),
                );
    
                Ok(to_send_token.unwrap())
            } else {
                Ok("Incorrect password".to_string())
            }
        } else {
            Ok("User not found".to_string())
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