use rocket::serde::json::Json;
use rocket::http::Status;
use rocket::response::{status::Created, status::Custom};
use crate::models::promotion_models::{Promotion, NewPromotion};

#[get("/promotions", format = "application/json")]
pub fn get_promotions() -> Result<Json<Vec<Promotion>>, Custom<&'static str>> {
    match Promotion::get_all_promotions() {
        Ok(promotions) => Ok(Json(promotions)),
        Err(_) => Err(Custom(Status::InternalServerError, "Failed retrieve promotions.")),
    }
}

#[get("/promotion/<id>", format = "application/json")]
pub fn get_promotion(id: i32) -> Result<Json<Promotion>, Custom<&'static str>> {
    match Promotion::get_promotion(id) {
        Ok(promotion) => Ok(Json(promotion)),
        Err(_) => Err(Custom(Status::InternalServerError, "Failed retrieve promotion.")),
    }
}

#[post("/promotion", format = "application/json", data = "<promotion>")]
pub fn post_promotion(promotion: Json<NewPromotion>) -> Result<Created<Json<&'static str>>, Custom<&'static str>> {
    match Promotion::insert_promotion(promotion.into_inner()) {
        Ok(_) => Ok(Created::new("/promotion").body(Json("promotion inserted successfully!"))),
        Err(_) => Err(Custom(Status::InternalServerError, "Failed insert promotion.")),
    }
}

#[patch("/promotion/<id>", format = "application/json", data = "<promotion>")]
pub fn update_promotion(id: i32, promotion: Json<NewPromotion>) -> Result<Created<Json<&'static str>>, Custom<&'static str>> {
    match Promotion::update_promotion(id, promotion.into_inner()) {
        Ok(_) => Ok(Created::new("/promotion/1").body(Json("promotion updated successfully!"))),
        Err(_) => Err(Custom(Status::InternalServerError, "Failed insert promotion.")),
    }
}

#[delete("/promotion/<id>")]
pub fn delete_promotion(id: i32) -> Result<Json<&'static str>, Custom<&'static str>> {    
    match Promotion::delete_promotion(id) {
        Ok(_) => Ok(Json("promotion deleted successfully!")),
        Err(_) => Err(Custom(Status::InternalServerError, "Failed retrieve promotion.")),
    }
}
