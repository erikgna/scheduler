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
extern crate rocket_cors;

use rocket::fs::FileServer;
use rocket::serde::json::Json;
use rocket::http::Method; // 1.
use rocket_cors::{
    AllowedHeaders, AllowedOrigins, Error, // 2.
    Cors, CorsOptions // 3.
};

use crate::routes::routes::authorization::{login, register, upload_user_image, delete_user_file, user_appointments, user_notifications, user_reviews, user_service_history, professional_profile, get_user};
use crate::routes::routes::professional::{get_professionals, get_professional, post_professional, update_professional, delete_professional, professional_services, professional_appointments, professional_reviews, professional_services_history};
use crate::routes::routes::service::{get_services, get_service, post_service, update_service, delete_service, upload_service_images, delete_service_file, service_appointments, service_reviews, service_services_history};
use crate::routes::routes::service_history::{get_services_history, get_service_history, post_service_history, update_service_history, delete_service_history};
use crate::routes::routes::review::{get_reviews, get_review, post_review, update_review, delete_review};
use crate::routes::routes::promotion::{get_promotions, get_promotion, post_promotion, update_promotion, delete_promotion};
use crate::routes::routes::notification::{get_notifications, get_notification, post_notification, update_notification, delete_notification};
use crate::routes::routes::appointment::{get_appointments, get_appointment, post_appointment, update_appointment, delete_appointment};

use crate::error_response::error_responses::{ErrorResponse, NOT_FOUND_JSON, UNAUTHORIZED_JSON, UNKNOWN_JSON};

pub mod db;
pub mod routes;
pub mod schema;
pub mod controllers;
pub mod models;
pub mod error_response;
pub mod utils;

#[rocket::launch]
fn rocket() -> _ {            
    let allowed_origins = AllowedOrigins::all();

    let cors = CorsOptions {
        allowed_origins,       
        allowed_headers: AllowedHeaders::all(),
        allow_credentials: true,
        ..Default::default()
    }
    .to_cors()
    .unwrap();    

    rocket::build()    
    .mount(
        "/api/v1", 
        routes![register, login, upload_user_image, delete_user_file, user_appointments, professional_profile, get_user, user_notifications, user_reviews, user_service_history,
        get_professionals, get_professional, post_professional, update_professional, delete_professional, professional_services, professional_appointments, professional_reviews, professional_services_history,
        get_services, get_service, post_service, update_service, delete_service, upload_service_images, delete_service_file, service_appointments, service_reviews, service_services_history, 
        get_services_history, get_service_history, post_service_history, update_service_history, delete_service_history,
        get_reviews, get_review, post_review, update_review, delete_review,
        get_promotions, get_promotion, post_promotion, update_promotion, delete_promotion,
        get_notifications, get_notification, post_notification, update_notification, delete_notification,
        get_appointments, get_appointment, post_appointment, update_appointment, delete_appointment,
    ])    
    .mount("/public", FileServer::from("public/"))
    .attach(cors)
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