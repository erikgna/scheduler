use rocket::serde::json::Json;
use rocket::http::Status;
use rocket::response::{status::Created, status::Custom};
use crate::models::appointment_models::{Appointment, NewAppointment};
use crate::models::user_models::AuthorizedUser;

#[get("/appointments/<page>/<page_size>?<date_time_appointment>&<appointment_status>", format = "application/json")]
pub fn get_appointments(
    page_size: i64, 
    page: i64,
    date_time_appointment: Option<String>,
    appointment_status: Option<String>
) -> Result<Json<Vec<Appointment>>, Custom<&'static str>> {
    match Appointment::get_all_appointments(page, page_size, date_time_appointment, appointment_status) {
        Ok(appointments) => Ok(Json(appointments)),
        Err(_) => Err(Custom(Status::InternalServerError, "Failed retrieve appointments.")),
    }
}

#[get("/appointment/<id>", format = "application/json")]
pub fn get_appointment(id: i32, _auth: AuthorizedUser) -> Result<Json<Appointment>, Custom<&'static str>> {
    match Appointment::get_appointment(id) {
        Ok(appointment) => Ok(Json(appointment)),
        Err(_) => Err(Custom(Status::InternalServerError, "Failed retrieve appointment.")),
    }
}

#[post("/appointment", format = "application/json", data = "<appointment>")]
pub fn post_appointment(appointment: Json<NewAppointment>, _auth: AuthorizedUser) -> Result<Created<Json<&'static str>>, Custom<&'static str>> {
    match Appointment::insert_appointment(appointment.into_inner()) {
        Ok(_) => Ok(Created::new("/appointment").body(Json("appointment inserted successfully!"))),
        Err(_) => Err(Custom(Status::InternalServerError, "Failed insert appointment.")),
    }
}

#[patch("/appointment/<id>", format = "application/json", data = "<appointment>")]
pub fn update_appointment(id: i32, appointment: Json<NewAppointment>, _auth: AuthorizedUser) -> Result<Created<Json<&'static str>>, Custom<&'static str>> {
    match Appointment::update_appointment(id, appointment.into_inner()) {
        Ok(_) => Ok(Created::new("/appointment/1").body(Json("appointment updated successfully!"))),
        Err(_) => Err(Custom(Status::InternalServerError, "Failed insert appointment.")),
    }
}

#[delete("/appointment/<id>")]
pub fn delete_appointment(id: i32, _auth: AuthorizedUser) -> Result<Json<&'static str>, Custom<&'static str>> {    
    match Appointment::delete_appointment(id) {
        Ok(_) => Ok(Json("appointment deleted successfully!")),
        Err(_) => Err(Custom(Status::InternalServerError, "Failed retrieve appointment.")),
    }
}
