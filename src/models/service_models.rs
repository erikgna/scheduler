use bigdecimal::BigDecimal;
use crate::schema::services;

#[derive(Queryable, Serialize)]
pub struct Service {
    pub id_service: i32,
    pub id_professional: Option<i32>,
    pub service_name: String,
    pub description: Option<String>,
    pub images: Option<String>,
    pub price: BigDecimal,
    pub duration: i32
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct NewService {    
    pub id_professional: Option<i32>,
    pub service_name: String,
    pub description: Option<String>,
    pub images: Option<String>,
    pub price: BigDecimal,
    pub duration: i32
}

#[derive(Insertable, PartialEq, Eq, Debug, Clone, AsChangeset)]
#[diesel(table_name = services)]
pub struct NewServiceInsert {    
    pub id_professional: Option<i32>,
    pub service_name: String,
    pub description: Option<String>,
    pub images: Option<String>,
    pub price: BigDecimal,
    pub duration: i32
}

impl From<NewService> for NewServiceInsert {
    fn from(service: NewService) -> Self {
        NewServiceInsert {
            id_professional: service.id_professional,
            service_name: service.service_name,
            description: service.description,
            images: service.images,
            price: service.price,            
            duration: service.duration,            
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PhotoResponse {
    pub photo_id: String,
    pub status: String,
}