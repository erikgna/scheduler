use bigdecimal::BigDecimal;
use crate::schema::services;

#[derive(Queryable, Serialize)]
pub struct Service {
    pub id_service: i32,
    pub service_name: String,
    pub description: Option<String>,
    pub images: Option<String>,
    pub price: BigDecimal
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct NewService {    
    pub service_name: String,
    pub description: Option<String>,
    pub images: Option<String>,
    pub price: BigDecimal
}

#[derive(Insertable, PartialEq, Eq, Debug, Clone, AsChangeset)]
#[diesel(table_name = services)]
pub struct NewServiceInsert {    
    pub service_name: String,
    pub description: Option<String>,
    pub images: Option<String>,
    pub price: BigDecimal
}

impl From<NewService> for NewServiceInsert {
    fn from(service: NewService) -> Self {
        NewServiceInsert {
            service_name: service.service_name,
            description: service.description,
            images: service.images,
            price: service.price,            
        }
    }
}