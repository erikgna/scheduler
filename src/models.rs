use diesel;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use argon2::Config;
use rand::Rng;
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
    pub fn insert_user(mut user: NewUser, conn: &PgConnection) -> bool {
        user.password = User::hash_password(user.password).unwrap();

        match diesel::insert_into(users::table)
        .values(&user)
        .execute(conn) {
            Ok(_) => true,
            Err(error) => {
                    println!("Error inserting user: {:?}", error);
                    false
            }
        }
    }

    pub fn login(auth: UserData, conn: &PgConnection) -> Result<bool, bool>  {
        use crate::schema::users::dsl::*;

        let user = users.filter(email.eq(auth.email)).first::<User>(conn);

        let password_match = User::verify_password(&auth.password, user.unwrap().password.as_bytes());

        if password_match {
            Ok(true)
        } else {
            println!("Password is incorrect");
            Err(false)
        }
    }

    pub fn hash_password(password: String) -> Option<String> {
        let salt: [u8; 32] = rand::thread_rng().gen();
        let config = Config::default();

        match argon2::hash_encoded(password.as_bytes(), &salt, &config){
            Ok(hash) => Some(hash),
            Err(_) => None,
        }
    }

    pub fn verify_password(password: &str, passwordEncoded: &[u8]) -> bool {
        argon2::verify_encoded(password, passwordEncoded).is_ok()
    }
}