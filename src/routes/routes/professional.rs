use rocket::serde::json::Json;
use rocket::http::Status;
use rocket::response::{status::Created, status::Custom, status};
use crate::models::professional_model::Professional;

#[get("/professionals", format = "application/json")]
pub fn get_professionals() -> Result<Json<Vec<Professional>>, Custom<&'static str>> {
    match Professional::get_all_professionals() {
        Ok(professionals) => Ok(Json(professionals)),
        // Ok(professionals) => {
        //     let string_json = serde_json::to_string(&professionals).unwrap();
        //     let json = Json(string_json);
        //     Ok(Created::new("/professionals").body(json))
        // }
        Err(_) => Err(Custom(Status::InternalServerError, "Failed retrieve professionals.")),
    }
}

// #[get("/teste")]
// pub fn teste(auth: AuthorizedUser) -> &'static str {
//     print!("User ID: {}", auth.user_id);
//     "Hello there! I'm a string!"
// }
