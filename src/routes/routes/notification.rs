use rocket::serde::json::Json;
use rocket::http::Status;
use rocket::response::{status::Created, status::Custom};
use crate::models::notification_models::{Notification, NewNotification};

#[get("/notifications", format = "application/json")]
pub fn get_notifications() -> Result<Json<Vec<Notification>>, Custom<&'static str>> {
    match Notification::get_all_notifications() {
        Ok(notifications) => Ok(Json(notifications)),
        Err(_) => Err(Custom(Status::InternalServerError, "Failed retrieve notifications.")),
    }
}

#[get("/notification/<id>", format = "application/json")]
pub fn get_notification(id: i32) -> Result<Json<Notification>, Custom<&'static str>> {
    match Notification::get_notification(id) {
        Ok(notification) => Ok(Json(notification)),
        Err(_) => Err(Custom(Status::InternalServerError, "Failed retrieve notification.")),
    }
}

#[post("/notification", format = "application/json", data = "<notification>")]
pub fn post_notification(notification: Json<NewNotification>) -> Result<Created<Json<&'static str>>, Custom<&'static str>> {
    match Notification::insert_notification(notification.into_inner()) {
        Ok(_) => Ok(Created::new("/notification").body(Json("notification inserted successfully!"))),
        Err(_) => Err(Custom(Status::InternalServerError, "Failed insert notification.")),
    }
}

#[patch("/notification/<id>", format = "application/json", data = "<notification>")]
pub fn update_notification(id: i32, notification: Json<NewNotification>) -> Result<Created<Json<&'static str>>, Custom<&'static str>> {
    match Notification::update_notification(id, notification.into_inner()) {
        Ok(_) => Ok(Created::new("/notification/1").body(Json("notification updated successfully!"))),
        Err(_) => Err(Custom(Status::InternalServerError, "Failed insert notification.")),
    }
}

#[delete("/notification/<id>")]
pub fn delete_notification(id: i32) -> Result<Json<&'static str>, Custom<&'static str>> {    
    match Notification::delete_notification(id) {
        Ok(_) => Ok(Json("notification deleted successfully!")),
        Err(_) => Err(Custom(Status::InternalServerError, "Failed retrieve notification.")),
    }
}
