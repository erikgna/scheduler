use std::env;

use crate::models::appointment_models::Appointment;
use crate::models::notification_models::Notification;
use crate::models::professional_models::Professional;
use crate::models::review_models::Review;
use crate::models::service_history_models::ServiceHistory;
use crate::models::user_models::{User, NewUser, UserLogin, UserToken, AuthorizedUser};
use crate::utils::file_utils::{save_file, delete_file};
use rocket::serde::json::Json;
use rocket::http::{Status, ContentType};
use rocket::response::{status::Created, status::Custom, status};
use rocket_multipart_form_data::{mime, MultipartFormDataError};

#[derive(Serialize)]
pub struct UserProfile {
    user: User,
    professional: Professional,
}

#[get("/professional-profile/<email>", format = "application/json")]
pub fn professional_profile(email: String) -> Result<Json<UserProfile>, Custom<&'static str>> {    
    match User::get_user_by_email(email) {
        Ok(user) => {
            let id = user.id;
            match Professional::get_user_professional(id) {
                Ok(professional) => {
                    let user_profile = UserProfile { user, professional };
                    Ok(Json(user_profile))
                },
                Err(_) => Err(Custom(Status::InternalServerError, "Failed to retrieve professional profile.")),
            }
        },
        Err(_) => Err(Custom(Status::InternalServerError, "Failed to retrieve user.")),
    }
}

#[get("/user-profile", format = "application/json")]
pub fn get_user(auth: AuthorizedUser) -> Result<Json<User>, Custom<&'static str>> {
    let int_user_id = match auth.user_id.parse::<i32>() {
        Ok(id) => id,
        Err(_) => {
            return Err(Custom(Status::InternalServerError, "Invalid user ID in the authorization header"));
        }
    };

    match User::get_user(int_user_id) {
        Ok(user) => Ok(Json(user)),
        Err(_) => Err(Custom(Status::InternalServerError, "Failed retrieve user.")),
    }
}

#[post("/register", format = "application/json", data = "<new_user>")]
pub fn register(new_user: Json<NewUser>) -> Result<Created<Json<&'static str>>, Custom<&'static str>> {
    match User::insert_user(new_user.into_inner()) {
        Ok(_) => Ok(Created::new("/user/1").body(Json("User registered successfully!"))),
        Err(_) => Err(Custom(Status::InternalServerError, "Failed to register user.")),
    }
}

#[post("/login", data = "<user_login>")]
pub fn login(user_login: Json<UserLogin>) -> Result<status::Custom<Json<UserToken>>, status::Custom<&'static str>> {
    match User::login(user_login.into_inner()) {
        Ok(token) => {
            let response = UserToken { token };
            Ok(status::Custom(Status::Ok, Json(response)))
        }
        Err(e) => {
            match e {
                diesel::result::Error::NotFound => {
                    Err(Custom(Status::NotFound, "User not found"))
                }
                _ => Err(Custom(Status::InternalServerError, "Failed to log in")),
            }
        }
    }
}

#[post("/user/upload", data = "<data>")]
pub async fn upload_user_image(
    content_type: &ContentType,
    data: rocket::Data<'_>,
    auth: AuthorizedUser
) -> Result<(), String> {
    let options = rocket_multipart_form_data::MultipartFormDataOptions::with_multipart_form_data_fields(vec![
        rocket_multipart_form_data::MultipartFormDataField::file("photo")
            .content_type_by_string(Some(mime::IMAGE_STAR))
            .unwrap(),
    ]);

    let multipart_form_data = match rocket_multipart_form_data::MultipartFormData::parse(content_type, data, options).await {
        Ok(data) => data,
        Err(MultipartFormDataError::DataTooLargeError(max_size)) => {
            return Err(format!("File size exceeds the limit of {} bytes", max_size));
        }
        Err(err) => {
            return Err(format!("Failed to parse form data: {}", err));
        }
    };

    let photo = match multipart_form_data.files.get("photo") {
        Some(file) => file,
        None => {
            return Err("No photo file found in the form data".to_string());
        }
    };

    let int_user_id = match auth.user_id.parse::<i32>() {
        Ok(id) => id,
        Err(_) => {
            return Err("Invalid user ID in the authorization header".to_string());
        }
    };

    let photo_path = format!("{}/users/{}", env::var("PUBLIC_PATH").unwrap_or("public".to_string()), int_user_id);

    match save_file(photo_path, Some(photo)).await {
        Ok(response) => {
            if let Err(err) = User::change_photo(response, int_user_id) {
                return Err(format!("Failed to update the user with the new photo: {}", err));
            }
            Ok(())
        }
        Err(err) => Err(format!("Failed to save the file: {}", err)),
    }
}

#[get("/user/delete/<filename>")]
pub fn delete_user_file(auth: AuthorizedUser, filename: String) -> Result<(), String> {
    let int_user_id = match auth.user_id.parse::<i32>() {
        Ok(id) => id,
        Err(_) => {
            return Err("Invalid user ID in the authorization header".to_string());
        }
    };

    let photo_path = format!("{}/users/{}/{}", env::var("PUBLIC_PATH").unwrap_or("public".to_string()), int_user_id, filename);
    match delete_file(photo_path.clone()){
        Ok(_) => {
            if let Err(err) = User::delete_photo(int_user_id) {
                return Err(format!("Failed to delete the user photo: {}", err));
            }
            Ok(())
        }
        Err(_) => Err(format!("Failed to delete the file")),
    }
}

#[get("/user/appointments", format = "application/json")]
pub fn user_appointments(auth: AuthorizedUser) -> Result<Json<Vec<Appointment>>, Custom<&'static str>> {
    let int_user_id = auth.user_id.parse::<i32>().unwrap_or(0);

    match Appointment::get_all_user_appointments(int_user_id) {
        Ok(appointments) => Ok(Json(appointments)),
        Err(_) => Err(Custom(Status::InternalServerError, "Failed retrieve appointments.")),
    }
}

#[get("/user/notifications", format = "application/json")]
pub fn user_notifications(auth: AuthorizedUser) -> Result<Json<Vec<Notification>>, Custom<&'static str>> {
    let int_user_id = auth.user_id.parse::<i32>().unwrap_or(0);

    match Notification::get_all_user_notifications(int_user_id) {
        Ok(notifications) => Ok(Json(notifications)),
        Err(_) => Err(Custom(Status::InternalServerError, "Failed retrieve notifications.")),
    }
}

#[get("/user/reviews", format = "application/json")]
pub fn user_reviews(auth: AuthorizedUser) -> Result<Json<Vec<Review>>, Custom<&'static str>> {
    let int_user_id = auth.user_id.parse::<i32>().unwrap_or(0);

    match Review::get_all_user_reviews(int_user_id) {
        Ok(reviews) => Ok(Json(reviews)),
        Err(_) => Err(Custom(Status::InternalServerError, "Failed retrieve reviews.")),
    }
}

#[get("/user/service-history", format = "application/json")]
pub fn user_service_history(auth: AuthorizedUser) -> Result<Json<Vec<ServiceHistory>>, Custom<&'static str>> {
    let int_user_id = auth.user_id.parse::<i32>().unwrap_or(0);

    match ServiceHistory::get_all_user_service_history(int_user_id) {
        Ok(service_history) => Ok(Json(service_history)),
        Err(_) => Err(Custom(Status::InternalServerError, "Failed retrieve service history.")),
    }
}
