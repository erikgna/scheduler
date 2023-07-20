use rocket::serde::json::Json;
use rocket::http::Status;
use rocket::response::{status::Created, status::Custom};
use crate::models::service_models::{Service, NewService};

#[get("/services", format = "application/json")]
pub fn get_services() -> Result<Json<Vec<Service>>, Custom<&'static str>> {
    match Service::get_all_services() {
        Ok(services) => Ok(Json(services)),
        Err(_) => Err(Custom(Status::InternalServerError, "Failed retrieve services.")),
    }
}

#[get("/service/<id>", format = "application/json")]
pub fn get_service(id: i32) -> Result<Json<Service>, Custom<&'static str>> {
    match Service::get_service(id) {
        Ok(service) => Ok(Json(service)),
        Err(_) => Err(Custom(Status::InternalServerError, "Failed retrieve service.")),
    }
}

#[post("/service", format = "application/json", data = "<service>")]
pub fn post_service(service: Json<NewService>) -> Result<Created<Json<&'static str>>, Custom<&'static str>> {
    match Service::insert_service(service.into_inner()) {
        Ok(_) => Ok(Created::new("/service").body(Json("Service inserted successfully!"))),
        Err(_) => Err(Custom(Status::InternalServerError, "Failed insert service.")),
    }
}

#[patch("/service/<id>", format = "application/json", data = "<service>")]
pub fn update_service(id: i32, service: Json<NewService>) -> Result<Created<Json<&'static str>>, Custom<&'static str>> {
    match Service::update_service(id, service.into_inner()) {
        Ok(_) => Ok(Created::new("/service/1").body(Json("Service updated successfully!"))),
        Err(_) => Err(Custom(Status::InternalServerError, "Failed insert service.")),
    }
}

#[delete("/service/<id>")]
pub fn delete_service(id: i32) -> Result<Json<&'static str>, Custom<&'static str>> {    
    match Service::delete_service(id) {
        Ok(_) => Ok(Json("Service deleted successfully!")),
        Err(_) => Err(Custom(Status::InternalServerError, "Failed retrieve service.")),
    }
}
