use rocket::serde::json::Json;
use rocket::http::Status;
use rocket::response::{status::Created, status::Custom};
use crate::models::professional_models::{Professional, NewProfessional};

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
