use std::io::Cursor;
use std::sync::atomic::{AtomicUsize, Ordering};

use rocket::{Request, Data, Response};
use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::{Method, ContentType, Status};

// Middleware para verificar a presença do token de autorização
pub struct AuthMiddleware;

#[rocket::async_trait]
impl Fairing for AuthMiddleware {
    fn info(&self) -> Info {
        Info {
            name: "Authorization Token Middleware",
            kind: Kind::Request,
        }
    }

    async fn on_request(&self, request: &mut Request<'_>, _: &mut Data<'_>) {
        if let Some(auth_header) = request.headers().get_one("Authorization") {
            if auth_header.starts_with("Bearer ") {
                print!("{}", auth_header);
                // O token está presente, prosseguir com a chamada da rota
                return;
            }
            
            let response = Response::build()
            .status(Status::Unauthorized)
            .finalize();
        }

        // Token ausente ou no formato inválido, retornar resposta de erro
        
        // request.set_abort();
        // request.set_response(
        //     rocket::Response::build()
        //         .status(Status::Unauthorized)
        //         .finalize(),
        // );
    }
}


