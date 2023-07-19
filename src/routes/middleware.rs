use rocket::Request;
use rocket::request::{FromRequest, Outcome};
use rocket::http::Status;
use jsonwebtoken::{decode, DecodingKey, Validation};
use crate::models::user_models::{Claims, AuthorizedUser};

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AuthorizedUser {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        if let Some(auth_header) = request.headers().get_one("Authorization") {
            let token = auth_header.split_whitespace().last().unwrap_or_default();

            match decode::<Claims>(
                token,
                &DecodingKey::from_secret("sua_chave_secreta".as_ref()),
                &Validation::default(),
            ) {
                Ok(claims) => {
                    Outcome::Success(AuthorizedUser {
                        user_id: claims.claims.id.to_string(),
                    })
                }
                Err(_) => Outcome::Failure((Status::Unauthorized, ())),
            }
        } else {
            Outcome::Failure((Status::Unauthorized, ()))
        }
    }
}


