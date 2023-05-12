use diesel::prelude::*;

#[derive(Queryable)]
pub struct User {
    pub id: Option<i32>,
    pub email: String,
    pub name: String,
    pub password: String,
}