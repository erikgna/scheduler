use std::env;

use rocket::serde::json::Json;
use rocket::http::{Status, ContentType};
use rocket::response::{status::Created, status::Custom};
use rocket_multipart_form_data::mime;
use crate::models::professional_models::{Professional, NewProfessional};
use crate::utils::file_utils::save_file;

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

#[post("/professional/upload/<id>", data = "<data>")]
pub async fn upload_professional_image(
    id: i32,
    content_type: &ContentType,
    data: rocket::Data<'_>    
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
    
    let photo_path = format!("{}/professionals/{}", env::var("PUBLIC_PATH").unwrap_or("public".to_string()), id);
    let response = save_file(photo_path, photo).await;
    let _ = Professional::change_photo(response, id);

    "Ok".to_string()    
}