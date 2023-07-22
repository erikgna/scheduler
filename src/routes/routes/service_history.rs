use rocket::serde::json::Json;
use rocket::http::Status;
use rocket::response::{status::Created, status::Custom};
use crate::models::service_history_models::{ServiceHistory, NewServiceHistory};

#[get("/services-history", format = "application/json")]
pub fn get_services_history() -> Result<Json<Vec<ServiceHistory>>, Custom<&'static str>> {
    match ServiceHistory::get_all_services_history() {
        Ok(services_history) => Ok(Json(services_history)),
        Err(_) => Err(Custom(Status::InternalServerError, "Failed retrieve services.")),
    }
}

#[get("/service-history/<id>", format = "application/json")]
pub fn get_service_history(id: i32) -> Result<Json<ServiceHistory>, Custom<&'static str>> {
    match ServiceHistory::get_service_history(id) {
        Ok(service_history) => Ok(Json(service_history)),
        Err(_) => Err(Custom(Status::InternalServerError, "Failed retrieve service.")),
    }
}

#[post("/service-history", format = "application/json", data = "<service_history>")]
pub fn post_service_history(service_history: Json<NewServiceHistory>) -> Result<Created<Json<&'static str>>, Custom<&'static str>> {
    match ServiceHistory::insert_service_history(service_history.into_inner()) {
        Ok(_) => Ok(Created::new("/service").body(Json("Service inserted successfully!"))),
        Err(_) => Err(Custom(Status::InternalServerError, "Failed insert service.")),
    }
}

#[patch("/service-history/<id>", format = "application/json", data = "<service_history>")]
pub fn update_service_history(id: i32, service_history: Json<NewServiceHistory>) -> Result<Created<Json<&'static str>>, Custom<&'static str>> {
    match ServiceHistory::update_service_history(id, service_history.into_inner()) {
        Ok(_) => Ok(Created::new("/service-history/1").body(Json("Service updated successfully!"))),
        Err(_) => Err(Custom(Status::InternalServerError, "Failed insert service.")),
    }
}

#[delete("/service-history/<id>")]
pub fn delete_service_history(id: i32) -> Result<Json<&'static str>, Custom<&'static str>> {    
    match ServiceHistory::delete_service_history(id) {
        Ok(_) => Ok(Json("Service deleted successfully!")),
        Err(_) => Err(Custom(Status::InternalServerError, "Failed retrieve service.")),
    }
}
