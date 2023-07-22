use crate::models::user_models::{User, NewUser, UserLogin, UserToken, AuthorizedUser};
use crate::utils::file_utils::save_file;
use rocket::serde::json::Json;
use rocket::http::{Status, ContentType};
use rocket::response::{status::Created, status::Custom, status};
use rocket_multipart_form_data::mime;

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

#[post("/upload", data = "<data>")]
pub async fn upload_image(
    content_type: &ContentType,
    data: rocket::Data<'_>,
    auth: AuthorizedUser
) -> String {
    let options = rocket_multipart_form_data::MultipartFormDataOptions::with_multipart_form_data_fields(vec![
        rocket_multipart_form_data::MultipartFormDataField::file("photo")
            .content_type_by_string(Some(mime::IMAGE_STAR))
            .unwrap(),
    ]);
    
    let multipart_form_data = match rocket_multipart_form_data::MultipartFormData::parse(content_type, data, options).await {
        Ok(data) => data,
        Err(err) => {
            return format!("Failed to parse form data: {}", err);
        }
    };

    let photo = multipart_form_data.files.get("photo");
    let response = save_file(photo).await;

    let int_user_id = auth.user_id.parse::<i32>().unwrap();    
    let _ = User::change_photo(response, int_user_id);

    "Ok".to_string()    
}
