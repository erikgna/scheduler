use rocket::Request;
use rocket::request::{FromRequest, Outcome};
use rocket::http::Status;

// Middleware para verificar a presença do token de autorização
pub struct AuthorizedUser {
    pub user_id: String,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AuthorizedUser {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let auth_header = request.headers().get_one("Authorization");

        match auth_header {
            Some(header) => {
                let token = header.split_whitespace().last().unwrap();
                let user_id = String::from("1");
                Outcome::Success(AuthorizedUser { user_id: user_id })
            }
            None => Outcome::Failure((Status::Unauthorized, ())),
        }        
    }
}


