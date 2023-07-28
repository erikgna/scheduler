use rocket::serde::json::Json;
use rocket::http::Status;
use rocket::response::{status::Created, status::Custom};
use crate::models::appointment_models::Appointment;
use crate::models::professional_models::{Professional, NewProfessional};
use crate::models::review_models::Review;
use crate::models::service_history_models::ServiceHistory;
use crate::models::service_models::Service;

#[get("/professionals", format = "application/json")]
pub fn get_professionals() -> Result<Json<Vec<Professional>>, Custom<&'static str>> {
    match Professional::get_all_professionals() {
        Ok(professionals) => Ok(Json(professionals)),
        Err(_) => Err(Custom(Status::InternalServerError, "Failed retrieve professionals.")),
    }
}

#[get("/professional/<id>", format = "application/json")]
pub fn get_professional(id: i32) -> Result<Json<Professional>, Custom<&'static str>> {
    match Professional::get_professional(id) {
        Ok(professional) => Ok(Json(professional)),
        Err(_) => Err(Custom(Status::InternalServerError, "Failed retrieve professional.")),
    }
}

#[post("/professional", format = "application/json", data = "<professional>")]
pub fn post_professional(professional: Json<NewProfessional>) -> Result<Created<Json<&'static str>>, Custom<&'static str>> {
    match Professional::insert_professional(professional.into_inner()) {
        Ok(_) => Ok(Created::new("/professional").body(Json("Professional inserted successfully!"))),
        Err(_) => Err(Custom(Status::InternalServerError, "Failed insert professional.")),
    }
}

#[patch("/professional/<id>", format = "application/json", data = "<professional>")]
pub fn update_professional(id: i32, professional: Json<NewProfessional>) -> Result<Created<Json<&'static str>>, Custom<&'static str>> {
    match Professional::update_professional(id, professional.into_inner()) {
        Ok(_) => Ok(Created::new("/professional/1").body(Json("Professional updated successfully!"))),
        Err(_) => Err(Custom(Status::InternalServerError, "Failed insert professional.")),
    }
}

#[delete("/professional/<id>")]
pub fn delete_professional(id: i32) -> Result<Json<&'static str>, Custom<&'static str>> {    
    match Professional::delete_professional(id) {
        Ok(_) => Ok(Json("Professional deleted successfully!")),
        Err(_) => Err(Custom(Status::InternalServerError, "Failed retrieve professional.")),
    }
}

#[get("/professional/<id>/services", format = "application/json")]
pub fn professional_services(id: i32) -> Result<Json<Vec<Service>>, Custom<&'static str>> {    
    match Service::get_all_professional_services(id) {
        Ok(services) => Ok(Json(services)),
        Err(_) => Err(Custom(Status::InternalServerError, "Failed retrieve services.")),
    }
}

#[get("/professional/<id>/appointments", format = "application/json")]
pub fn professional_appointments(id: i32) -> Result<Json<Vec<Appointment>>, Custom<&'static str>> {    
    match Appointment::get_all_professional_appointments(id) {
        Ok(appointments) => Ok(Json(appointments)),
        Err(_) => Err(Custom(Status::InternalServerError, "Failed retrieve appointments.")),
    }
}

#[get("/professional/<id>/reviews", format = "application/json")]
pub fn professional_reviews(id: i32) -> Result<Json<Vec<Review>>, Custom<&'static str>> {    
    match Review::get_all_professional_reviews(id) {
        Ok(reviews) => Ok(Json(reviews)),
        Err(_) => Err(Custom(Status::InternalServerError, "Failed retrieve reviews.")),
    }
}

#[get("/professional/<id>/services-history", format = "application/json")]
pub fn professional_services_history(id: i32) -> Result<Json<Vec<ServiceHistory>>, Custom<&'static str>> {    
    match ServiceHistory::get_all_professional_service_history(id) {
        Ok(services_history) => Ok(Json(services_history)),
        Err(_) => Err(Custom(Status::InternalServerError, "Failed retrieve reviews.")),
    }
}