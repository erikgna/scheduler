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
use rocket::serde::json::Json;

use crate::routes::routes::authorization::{login, register};
use crate::routes::routes::professional::{get_professionals, get_professional, post_professional, update_professional, delete_professional};
use crate::routes::routes::service::{get_services, get_service, post_service, update_service, delete_service};
use crate::error_response::error_responses::{
    ErrorResponse, NOT_FOUND_JSON, UNAUTHORIZED_JSON, UNKNOWN_JSON,
};

pub mod db;
pub mod routes;
pub mod schema;
pub mod controllers;
pub mod models;
pub mod error_response;

#[rocket::launch]
fn rocket() -> _ {        
    rocket::build()
    .mount(
        "/api/v1/", 
        routes![register, login, 
        get_professionals, get_professional, post_professional, update_professional, delete_professional, 
        get_services, get_service, post_service, update_service, delete_service
    ])    
    .register("/", catchers![unauthorized, not_found, internal_sever_error])
}

#[catch(401)]
pub fn unauthorized() -> Json<ErrorResponse> {
    Json(UNAUTHORIZED_JSON)
}

#[catch(404)]
pub fn not_found() -> Json<ErrorResponse> {
    Json(NOT_FOUND_JSON)
}

#[catch(500)]
pub fn internal_sever_error() -> Json<ErrorResponse> {
    Json(UNKNOWN_JSON)
}