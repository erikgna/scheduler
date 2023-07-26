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
        rocket_multipart_form_data::MultipartFormDataField::file("photo3")
            .content_type_by_string(Some(mime::IMAGE_STAR))
            .unwrap(),
        rocket_multipart_form_data::MultipartFormDataField::file("photo4")
            .content_type_by_string(Some(mime::IMAGE_STAR))
            .unwrap(),
        rocket_multipart_form_data::MultipartFormDataField::file("photo5")
            .content_type_by_string(Some(mime::IMAGE_STAR))
            .unwrap()
    ]);

    let multipart_form_data = match rocket_multipart_form_data::MultipartFormData::parse(content_type, data, options).await {
        Ok(data) => data,
        Err(err) => {
            return format!("Failed to parse form data: {}", err);
        }
    };

    let photo_path = format!("{}/services/{}", env::var("PUBLIC_PATH").unwrap_or("public".to_string()), id);

    let mut responses = Vec::new();

    let photo = multipart_form_data.files.get("photo");
    responses.push(save_file(photo_path.clone(), photo).await);    

    let photo1 = multipart_form_data.files.get("photo1");
    responses.push(save_file(photo_path.clone(), photo1).await);

    let photo2: Option<&Vec<rocket_multipart_form_data::FileField>> = multipart_form_data.files.get("photo2");
    responses.push(save_file(photo_path.clone(), photo2).await);

    let photo3 = multipart_form_data.files.get("photo3");
    responses.push(save_file(photo_path.clone(), photo3).await);

    let photo4 = multipart_form_data.files.get("photo4");
    responses.push(save_file(photo_path.clone(), photo4).await);

    let photo5 = multipart_form_data.files.get("photo5");
    responses.push(save_file(photo_path.clone(), photo5).await);

    let json_string = serde_json::to_string(&responses).unwrap();
    let _ = Service::change_photo(json_string, id);

    "Ok".to_string()
}
