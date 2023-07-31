use std::env;

use rocket::serde::json::Json;
use rocket::http::{Status, ContentType};
use rocket::response::{status::Created, status::Custom};
use crate::models::appointment_models::Appointment;
use crate::models::review_models::Review;
use crate::models::service_history_models::ServiceHistory;
use crate::models::service_models::{Service, NewService, PhotoResponse};
use crate::models::user_models::AuthorizedUser;
use crate::utils::file_utils::{save_file, delete_file};

#[get("/services/<page>/<page_size>?<service_name>&<price>&<duration>", format = "application/json")]
pub fn get_services(
    page_size: i64, 
    page: i64,
    service_name: Option<String>,
    price: Option<String>,
    duration: Option<String>
) -> Result<Json<Vec<Service>>, Custom<&'static str>> {
    match Service::get_all_services(page, page_size, service_name, price, duration) {
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
pub fn post_service(service: Json<NewService>, _auth: AuthorizedUser) -> Result<Created<Json<&'static str>>, Custom<&'static str>> {
    match Service::insert_service(service.into_inner()) {
        Ok(_) => Ok(Created::new("/service").body(Json("Service inserted successfully!"))),
        Err(_) => Err(Custom(Status::InternalServerError, "Failed insert service.")),
    }
}

#[patch("/service/<id>", format = "application/json", data = "<service>")]
pub fn update_service(id: i32, service: Json<NewService>, _auth: AuthorizedUser) -> Result<Created<Json<&'static str>>, Custom<&'static str>> {
    match Service::update_service(id, service.into_inner()) {
        Ok(_) => Ok(Created::new("/service/1").body(Json("Service updated successfully!"))),
        Err(_) => Err(Custom(Status::InternalServerError, "Failed insert service.")),
    }
}

#[delete("/service/<id>")]
pub fn delete_service(id: i32, _auth: AuthorizedUser) -> Result<Json<&'static str>, Custom<&'static str>> {    
    match Service::delete_service(id) {
        Ok(_) => Ok(Json("Service deleted successfully!")),
        Err(_) => Err(Custom(Status::InternalServerError, "Failed retrieve service.")),
    }
}

use rocket_multipart_form_data::{mime, MultipartFormDataError};
#[post("/service/upload/<id>", data = "<data>")]
pub async fn upload_service_images(
    id: i32,
    content_type: &ContentType,
    data: rocket::Data<'_>,
    _auth: AuthorizedUser
) -> Result<String, String> {
    let options = rocket_multipart_form_data::MultipartFormDataOptions::with_multipart_form_data_fields(vec![
        rocket_multipart_form_data::MultipartFormDataField::file("photo0")
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
        Err(MultipartFormDataError::DataTooLargeError(max_size)) => {
            return Err(format!("File size exceeds the limit of {} bytes", max_size));
        }
        Err(err) => {
            return Err(format!("Failed to parse form data: {}", err));
        }
    };

    let photo_path = format!("{}/services/{}", env::var("PUBLIC_PATH").unwrap_or("public".to_string()), id);
    let mut responses = Vec::new();

    for i in 0..6 {
        let photo_key = format!("photo{}", i);
        let photo = match multipart_form_data.files.get(photo_key.trim()) {
            Some(file) => file,
            None => { continue }
        };

        let response = save_file(photo_path.clone(), Some(photo)).await;
        let status = match &response {
            Ok(_) => "Ok".to_string(),
            Err(_) => format!("No file"),
        };

        responses.push(PhotoResponse {
            photo_id: response.unwrap(),
            status,
        });
    }

    let json_string = serde_json::to_string(&responses).map_err(|err| format!("Failed to serialize responses: {}", err))?;
    Service::change_photo(json_string.clone(), id).map_err(|err| format!("Failed to update the service with the new photo: {}", err))?;

    Ok(json_string)
}

#[get("/service/delete/<id>/<filename>")]
pub fn delete_service_file(id: i32, filename: String, _auth: AuthorizedUser) -> Result<(), String> {   
    let photo_path = format!("{}/services/{}/{}", env::var("PUBLIC_PATH").unwrap_or("public".to_string()), id, filename);

    match Service::get_service(id) {
        Ok(service) => {
            let images_str = service.images.unwrap();
            // Fazer o parse da string JSON para um vetor de objetos ImageItem
            let mut images: Vec<PhotoResponse> = serde_json::from_str(&images_str).unwrap();

            // Encontrar o índice do objeto que possui o filename desejado
            let index = images.iter().position(|item| item.photo_id == filename);

            // Verificar se o objeto foi encontrado e, se sim, removê-lo
            if let Some(index) = index {
                images.remove(index);
            } else {
                return Err(format!("File with filename {} not found in images", filename));
            }

            // Converter o vetor atualizado de volta para uma string JSON
            let updated_images_str = serde_json::to_string(&images)
                .map_err(|err| format!("Failed to convert images to JSON: {}", err))?;

            delete_file(photo_path.clone()).map_err(|err| format!("Failed to update the service with the new photo: {}", err))?;
                
            if let Err(err) = Service::change_photo(updated_images_str.clone(), id) {
                return Err(format!("Failed to delete the service photo: {}", err));
            }                                                     

            Ok(())
        },
        Err(_) => Err(format!("Failed to delete the file")),
    }
}

#[get("/service/<id>/appointments", format = "application/json")]
pub fn service_appointments(id: i32) -> Result<Json<Vec<Appointment>>, Custom<&'static str>> {    
    match Appointment::get_all_service_appointments(id) {
        Ok(appointments) => Ok(Json(appointments)),
        Err(_) => Err(Custom(Status::InternalServerError, "Failed retrieve appointments.")),
    }
}

#[get("/service/<id>/reviews", format = "application/json")]
pub fn service_reviews(id: i32) -> Result<Json<Vec<Review>>, Custom<&'static str>> {    
    match Review::get_all_service_reviews(id) {
        Ok(reviews) => Ok(Json(reviews)),
        Err(_) => Err(Custom(Status::InternalServerError, "Failed retrieve reviews.")),
    }
}

#[get("/service/<id>/services-history", format = "application/json")]
pub fn service_services_history(id: i32, _auth: AuthorizedUser) -> Result<Json<Vec<ServiceHistory>>, Custom<&'static str>> {    
    match ServiceHistory::get_all_service_service_history(id) {
        Ok(services_history) => Ok(Json(services_history)),
        Err(_) => Err(Custom(Status::InternalServerError, "Failed retrieve services_history.")),
    }
}