use diesel;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use super::schema::users;
// this is to get users from the database
#[derive(Serialize, Queryable)] 
pub struct User {
    pub id: i32,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub token: String,
    pub password: String
}

// decode request data
#[derive(Deserialize)] 
pub struct UserData {
    pub email: String,
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
    pub fn insert_user(user: NewUser, conn: &PgConnection) -> bool {
        diesel::insert_into(users::table)
            .values(&user)
            .execute(conn)
            .is_ok()
    }

    pub fn get_user_by_email(target_email: &str, conn: &PgConnection) -> Result<User, diesel::result::Error>  {
        use crate::schema::users::dsl::*;

        users.filter(email.eq(target_email))
            .first(conn)
    }
}