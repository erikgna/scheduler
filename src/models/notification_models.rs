use chrono::NaiveDateTime;
use crate::schema::notifications;

#[derive(Queryable, Serialize)]
pub struct Notification {
    pub id_notification: i32,
    pub id_user: Option<i32>,
    pub id_professional: Option<i32>,
    pub message: String,
    pub date_time_sent: NaiveDateTime
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct NewNotification {    
    pub id_user: Option<i32>,
    pub id_professional: Option<i32>,
    pub message: String,
    pub date_time_sent: NaiveDateTime
}

#[derive(Insertable, PartialEq, Eq, Debug, Clone, AsChangeset)]
#[diesel(table_name = notifications)]
pub struct NewNotificationInsert {        
    pub id_user: Option<i32>,
    pub id_professional: Option<i32>,
    pub message: String,
    pub date_time_sent: NaiveDateTime
}

impl From<NewNotification> for NewNotificationInsert {
    fn from(notification: NewNotification) -> Self {
        NewNotificationInsert {
            id_user: notification.id_user,
            id_professional: notification.id_professional,
            message: notification.message,
            date_time_sent: notification.date_time_sent
        }
    }
}