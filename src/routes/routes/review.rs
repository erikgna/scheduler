use rocket::serde::json::Json;
use rocket::http::Status;
use rocket::response::{status::Created, status::Custom};
use crate::models::review_models::{Review, NewReview};

#[get("/reviews", format = "application/json")]
pub fn get_reviews() -> Result<Json<Vec<Review>>, Custom<&'static str>> {
    match Review::get_all_reviews() {
        Ok(reviews) => Ok(Json(reviews)),
        Err(_) => Err(Custom(Status::InternalServerError, "Failed retrieve reviews.")),
    }
}

#[get("/review/<id>", format = "application/json")]
pub fn get_review(id: i32) -> Result<Json<Review>, Custom<&'static str>> {
    match Review::get_review(id) {
        Ok(review) => Ok(Json(review)),
        Err(_) => Err(Custom(Status::InternalServerError, "Failed retrieve review.")),
    }
}

#[post("/review", format = "application/json", data = "<review>")]
pub fn post_review(review: Json<NewReview>) -> Result<Created<Json<&'static str>>, Custom<&'static str>> {
    match Review::insert_review(review.into_inner()) {
        Ok(_) => Ok(Created::new("/review").body(Json("review inserted successfully!"))),
        Err(_) => Err(Custom(Status::InternalServerError, "Failed insert review.")),
    }
}

#[patch("/review/<id>", format = "application/json", data = "<review>")]
pub fn update_review(id: i32, review: Json<NewReview>) -> Result<Created<Json<&'static str>>, Custom<&'static str>> {
    match Review::update_review(id, review.into_inner()) {
        Ok(_) => Ok(Created::new("/review/1").body(Json("review updated successfully!"))),
        Err(_) => Err(Custom(Status::InternalServerError, "Failed insert review.")),
    }
}

#[delete("/review/<id>")]
pub fn delete_review(id: i32) -> Result<Json<&'static str>, Custom<&'static str>> {    
    match Review::delete_review(id) {
        Ok(_) => Ok(Json("review deleted successfully!")),
        Err(_) => Err(Custom(Status::InternalServerError, "Failed retrieve review.")),
    }
}
