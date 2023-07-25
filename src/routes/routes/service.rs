use std::env;

use rocket::serde::json::Json;
use rocket::http::{Status, ContentType};
use rocket::response::{status::Created, status::Custom};
use crate::models::service_models::{Service, NewService};
use crate::utils::file_utils::save_file;

#[get("/services", format = "application/json")]
pub fn get_services() -> Result<Json<Vec<Service>>, Custom<&'static str>> {
    match Service::get_all_services() {
        Ok(services) => Ok(Json(services)),
        Err(_) => Err(Custom(Status::InternalServerError, "Failed retrieve services.")),
    }
}

#[get("/service/<id>", format = "application/json")]
pub fn get_service(id: i32) -> Result<Json<Service>, Custom<&'static str>> {
    match Service::get_service(id) {
        Ok(service) => Ok(Json(service)),
        Err(_) => Err(Custom(Status::InternalServerError, "Failed retrieve service.")),
    }
}

#[post("/service", format = "application/json", data = "<service>")]
pub fn post_service(service: Json<NewService>) -> Result<Created<Json<&'static str>>, Custom<&'static str>> {
    match Service::insert_service(service.into_inner()) {
        Ok(_) => Ok(Created::new("/service").body(Json("Service inserted successfully!"))),
        Err(_) => Err(Custom(Status::InternalServerError, "Failed insert service.")),
    }
}

#[patch("/service/<id>", format = "application/json", data = "<service>")]
pub fn update_service(id: i32, service: Json<NewService>) -> Result<Created<Json<&'static str>>, Custom<&'static str>> {
    match Service::update_service(id, service.into_inner()) {
        Ok(_) => Ok(Created::new("/service/1").body(Json("Service updated successfully!"))),
        Err(_) => Err(Custom(Status::InternalServerError, "Failed insert service.")),
    }
}

#[delete("/service/<id>")]
pub fn delete_service(id: i32) -> Result<Json<&'static str>, Custom<&'static str>> {    
    match Service::delete_service(id) {
        Ok(_) => Ok(Json("Service deleted successfully!")),
        Err(_) => Err(Custom(Status::InternalServerError, "Failed retrieve service.")),
    }
}

use rocket_multipart_form_data::mime;

#[post("/service/upload/<id>", data = "<data>")]
pub async fn upload_service_images(
    id: i32,
    content_type: &ContentType,
    data: rocket::Data<'_>,
) -> String {
    let options = rocket_multipart_form_data::MultipartFormDataOptions::with_multipart_form_data_fields(vec![
        rocket_multipart_form_data::MultipartFormDataField::file("photo")
            .content_type_by_string(Some(mime::IMAGE_STAR))
            .unwrap(),
        rocket_multipart_form_data::MultipartFormDataField::file("photo1")
            .content_type_by_string(Some(mime::IMAGE_STAR))
            .unwrap(),
        rocket_multipart_form_data::MultipartFormDataField::file("photo2")
            .content_type_by_string(Some(mime::IMAGE_STAR))
            .unwrap(),
        // Add more fields as needed
    ]);

    let multipart_form_data = match rocket_multipart_form_data::MultipartFormData::parse(content_type, data, options).await {
        Ok(data) => data,
        Err(err) => {
            return format!("Failed to parse form data: {}", err);
        }
    };

    if let Some(file_fields) = multipart_form_data.files.get("photo") {
        print!("{:?}", file_fields);
    }

    if let Some(file_fields) = multipart_form_data.files.get("photo1") {
        print!("{:?}", file_fields);
    }

    if let Some(file_fields) = multipart_form_data.files.get("photo2") {
        print!("{:?}", file_fields);
    }

    // Process other fields as needed

    "Ok".to_string()
}
