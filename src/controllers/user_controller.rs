use diesel;
use diesel::prelude::*;
use crate::schema::users;

use argon2::{Config, hash_encoded};
use rand::Rng;
use rand::distributions::Alphanumeric;
use jsonwebtoken::{encode, Header, EncodingKey};

use crate::{db::establish_connection, models::user_models::{User, NewUser, NewUserInsert, UserLogin, Claims}};

impl User {
    pub fn get_user(user_id: i32) -> Result<User, diesel::result::Error> {
        use crate::schema::users::dsl::*;

        let conn = &mut establish_connection();        
        users.filter(id.eq(user_id)).first::<User>(conn)
    }

    pub fn get_user_by_email(user_email: String) -> Result<User, diesel::result::Error> {
        use crate::schema::users::dsl::*;

        let conn = &mut establish_connection();        
        users.filter(email.eq(user_email)).first::<User>(conn)
    }

    pub fn change_photo(photo_path: String, user_id: i32) -> Result<(), diesel::result::Error>{
        let conn = &mut establish_connection();
        
        conn.transaction(|conn| {
            diesel::update(users::table.filter(users::id.eq(user_id)))
                .set(users::photo.eq(photo_path))
                .execute(conn)?;
            
            Ok(())
        })
    }

    pub fn delete_photo(user_id: i32) -> Result<(), diesel::result::Error>{
        let conn = &mut establish_connection();
        
        conn.transaction(|conn| {
            diesel::update(users::table.filter(users::id.eq(user_id)))
                .set(users::photo.eq(""))
                .execute(conn)?;
            
            Ok(())
        })
    } 

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
                    role: user.role.clone(),
                    exp: 1844674407
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