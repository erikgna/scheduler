#![feature(plugin, decl_macro, proc_macro_hygiene)]
#![allow(proc_macro_derive_resolution_fallback, unused_attributes)]

#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate r2d2;
extern crate r2d2_diesel;
#[macro_use]
extern crate rocket;
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;

use routes::{login, teste, register};

mod db;
mod routes;
mod models;
mod schema;
mod middleware;
mod user_models;

#[rocket::launch]
fn rocket() -> _ {    
    rocket::build()
    .mount("/api/v1/", routes![register, login, teste])
}