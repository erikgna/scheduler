use rocket::serde::json::Json;
use rocket::http::Status;
use rocket::response::{status::Created, status::Custom};
use crate::models::notification_models::{Notification, NewNotification};
use crate::models::user_models::AuthorizedUser;

#[get("/notifications/<page>/<page_size>?<message>&<date_time_sent>", format = "application/json")]
pub fn get_notifications(
    _auth: AuthorizedUser,
    page_size: i64, 
    page: i64,
    message: Option<String>,
    date_time_sent: Option<String>
) -> Result<Json<Vec<Notification>>, Custom<&'static str>> {
    match Notification::get_all_notifications(page, page_size, message, date_time_sent) {
        Ok(notifications) => Ok(Json(notifications)),
        Err(_) => Err(Custom(Status::InternalServerError, "Failed retrieve notifications.")),
    }
}

#[get("/notification/<id>", format = "application/json")]
pub fn get_notification(id: i32, _auth: AuthorizedUser) -> Result<Json<Notification>, Custom<&'static str>> {
    match Notification::get_notification(id) {
        Ok(notification) => Ok(Json(notification)),
        Err(_) => Err(Custom(Status::InternalServerError, "Failed retrieve notification.")),
    }
}

#[post("/notification", format = "application/json", data = "<notification>")]
pub fn post_notification(notification: Json<NewNotification>, _auth: AuthorizedUser) -> Result<Created<Json<&'static str>>, Custom<&'static str>> {
    match Notification::insert_notification(notification.into_inner()) {
        Ok(_) => Ok(Created::new("/notification").body(Json("notification inserted successfully!"))),
        Err(_) => Err(Custom(Status::InternalServerError, "Failed insert notification.")),
    }
}

#[patch("/notification/<id>", format = "application/json", data = "<notification>")]
pub fn update_notification(id: i32, notification: Json<NewNotification>, _auth: AuthorizedUser) -> Result<Created<Json<&'static str>>, Custom<&'static str>> {
    match Notification::update_notification(id, notification.into_inner()) {
        Ok(_) => Ok(Created::new("/notification/1").body(Json("notification updated successfully!"))),
        Err(_) => Err(Custom(Status::InternalServerError, "Failed insert notification.")),
    }
}

#[delete("/notification/<id>")]
pub fn delete_notification(id: i32, _auth: AuthorizedUser) -> Result<Json<&'static str>, Custom<&'static str>> {    
    match Notification::delete_notification(id) {
        Ok(_) => Ok(Json("notification deleted successfully!")),
        Err(_) => Err(Custom(Status::InternalServerError, "Failed retrieve notification.")),
    }
}
