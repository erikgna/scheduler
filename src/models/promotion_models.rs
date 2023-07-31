use chrono::NaiveDate;
use crate::schema::promotions;

#[derive(Serialize)]
pub struct PromotionsResponse {
    pub total_docs: i64,
    pub result: Vec<Promotion>,
}

#[derive(Queryable, Serialize)]
pub struct Promotion {
    pub id_promotion: i32,
    pub promotion_name: String,
    pub description: Option<String>,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub discount_code: Option<String>
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct NewPromotion {    
    pub promotion_name: String,
    pub description: Option<String>,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub discount_code: Option<String>
}

#[derive(Insertable, PartialEq, Eq, Debug, Clone, AsChangeset)]
#[diesel(table_name = promotions)]
pub struct NewPromotionInsert {        
    pub promotion_name: String,
    pub description: Option<String>,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub discount_code: Option<String>
}

impl From<NewPromotion> for NewPromotionInsert {
    fn from(promotion: NewPromotion) -> Self {
        NewPromotionInsert {
            promotion_name: promotion.promotion_name,
            description: promotion.description,
            start_date: promotion.start_date,
            end_date: promotion.end_date,
            discount_code: promotion.discount_code
        }
    }
}