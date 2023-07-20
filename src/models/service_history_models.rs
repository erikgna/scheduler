use crate::schema::service_history;
use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;

#[derive(Queryable, Serialize)]
pub struct ServiceHistory {
    pub id_record: i32,
    pub id_user: Option<i32>,
    pub id_service: Option<i32>,
    pub date_time_service: NaiveDateTime,
    pub amount_paid: BigDecimal,
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct NewServiceHistory {    
    pub id_user: Option<i32>,
    pub id_service: Option<i32>,
    pub date_time_service: NaiveDateTime,
    pub amount_paid: BigDecimal,
}

#[derive(Insertable, PartialEq, Eq, Debug, Clone, AsChangeset)]
#[diesel(table_name = service_history)]
pub struct NewServiceHistoryInsert {
    pub id_user: Option<i32>,
    pub id_service: Option<i32>,
    pub date_time_service: NaiveDateTime,
    pub amount_paid: BigDecimal,
}

impl From<NewServiceHistory> for NewServiceHistoryInsert {
    fn from(service_history: NewServiceHistory) -> Self {
        NewServiceHistoryInsert {
            id_user: service_history.id_user,
            id_service: service_history.id_service,
            date_time_service: service_history.date_time_service,
            amount_paid: service_history.amount_paid
        }
    }
}