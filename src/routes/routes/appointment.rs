use rocket::serde::json::Json;
use rocket::http::Status;
use rocket::response::{status::Created, status::Custom};
use crate::models::appointment_models::{Appointment, NewAppointment};

#[get("/appointments", format = "application/json")]
pub fn get_appointments() -> Result<Json<Vec<Appointment>>, Custom<&'static str>> {
    match Appointment::get_all_appointments() {
        Ok(appointments) => Ok(Json(appointments)),
        Err(_) => Err(Custom(Status::InternalServerError, "Failed retrieve appointments.")),
    }
}

#[get("/appointment/<id>", format = "application/json")]
pub fn get_appointment(id: i32) -> Result<Json<Appointment>, Custom<&'static str>> {
    match Appointment::get_appointment(id) {
        Ok(appointment) => Ok(Json(appointment)),
        Err(_) => Err(Custom(Status::InternalServerError, "Failed retrieve appointment.")),
    }
}

#[post("/appointment", format = "application/json", data = "<appointment>")]
pub fn post_appointment(appointment: Json<NewAppointment>) -> Result<Created<Json<&'static str>>, Custom<&'static str>> {
    match Appointment::insert_appointment(appointment.into_inner()) {
        Ok(_) => Ok(Created::new("/appointment").body(Json("appointment inserted successfully!"))),
        Err(_) => Err(Custom(Status::InternalServerError, "Failed insert appointment.")),
    }
}

#[patch("/appointment/<id>", format = "application/json", data = "<appointment>")]
pub fn update_appointment(id: i32, appointment: Json<NewAppointment>) -> Result<Created<Json<&'static str>>, Custom<&'static str>> {
    match Appointment::update_appointment(id, appointment.into_inner()) {
        Ok(_) => Ok(Created::new("/appointment/1").body(Json("appointment updated successfully!"))),
        Err(_) => Err(Custom(Status::InternalServerError, "Failed insert appointment.")),
    }
}

#[delete("/appointment/<id>")]
pub fn delete_appointment(id: i32) -> Result<Json<&'static str>, Custom<&'static str>> {    
    match Appointment::delete_appointment(id) {
        Ok(_) => Ok(Json("appointment deleted successfully!")),
        Err(_) => Err(Custom(Status::InternalServerError, "Failed retrieve appointment.")),
    }
}
