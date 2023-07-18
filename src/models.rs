use diesel;
use diesel::prelude::*;
use argon2::{Config, hash_encoded};
use rand::Rng;
use rand::distributions::Alphanumeric;
use crate::{db::establish_connection, user_models::UserLogin, user_models::NewUser, user_models::NewUserInsert};
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

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    id: i32,        // ID do usuário
    email: String,  // Email do usuário
}

impl User {
    pub fn insert_user(user: NewUser) -> Result<(), diesel::result::Error> {
        let conn = &mut establish_connection();
    
        let pass = User::hash_password(&user.password);
        let user_insert = NewUserInsert::from(user); // Convert NewUser to NewUserInsert
        let user_insert_with_hashed_pass = NewUserInsert {
            password: pass.unwrap(), // Update the password field with the hashed password
            ..user_insert
        };
    
        match diesel::insert_into(users::table)
            .values(&user_insert_with_hashed_pass)
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
        match argon2::verify_encoded(password, password_encoded){
            Ok(true) => true,
            Ok(false) => false,
            Err(error) => {
                println!("Error verifying password: {:?}", error);
                false
            }
        }
    }
    
    pub fn login(auth: UserLogin) -> Result<String, diesel::result::Error> {
        use crate::schema::users::dsl::*;
        let conn = &mut establish_connection();        
        
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