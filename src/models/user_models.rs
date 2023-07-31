use crate::schema::users;

#[derive(Serialize, Queryable)] 
pub struct User {
    pub id: i32,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub phone: String,
    pub address: String,
    pub address_number: String,
    pub city: String,
    pub state: String,
    pub password: String,
    pub role: i32,
    pub photo: Option<String>,
    pub token: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub id: i32,        // ID do usuário
    pub email: String,  // Email do usuário
    pub role: i32,
    pub exp: usize
}

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
    pub photo: String,
    pub phone: String,
    pub address: String,
    pub address_number: String,
    pub state: String,
    pub city: String,
    pub role: i32,
}

#[derive(Insertable, PartialEq, Eq, Debug, Clone)]
#[diesel(table_name = users)]
pub struct NewUserInsert {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
    pub photo: String,
    pub phone: String,
    pub address: String,
    pub address_number: String,
    pub state: String,
    pub city: String,
    pub role: i32,
}

// Implement a conversion function from `NewUser` to `NewUserInsert`
impl From<NewUser> for NewUserInsert {
    fn from(user: NewUser) -> Self {
        NewUserInsert {
            first_name: user.first_name,
            last_name: user.last_name,
            email: user.email,
            password: user.password,
            phone: user.phone,
            photo: user.photo,
            address: user.address,
            address_number: user.address_number,
            city: user.city,
            state: user.state,
            role: user.role,
        }
    }
}

pub struct AuthorizedUser {
    pub user_id: String,
    pub user_role: i32,
}
