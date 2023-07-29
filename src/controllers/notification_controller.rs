use chrono::NaiveDateTime;
use diesel;
use diesel::prelude::*;
use crate::{schema::notifications, models::notification_models::{Notification, NewNotification, NewNotificationInsert}};
use crate::db::establish_connection;

impl Notification {
    pub fn get_notification(id: i32) -> Result<Notification, diesel::result::Error> {
        use crate::schema::notifications::dsl::*;

        let conn = &mut establish_connection();
        notifications.filter(id_notification.eq(id)).first::<Notification>(conn)        
    }

    pub fn get_all_notifications(
        page_size: i64, 
        page: i64,
        message_filter: Option<String>,
        date_time_sent_filter: Option<String>
    ) -> Result<Vec<Notification>, diesel::result::Error> {
        use crate::schema::notifications::dsl::*;
        
        let conn = &mut establish_connection();        
        let mut query = notifications.into_boxed();
        
        if let Some(msg_filter) = message_filter {
            query = query.filter(message.eq(msg_filter));
        }        

        if let Some(date_time_sent_f) = date_time_sent_filter {            
            query = query.filter(date_time_sent.le(NaiveDateTime::parse_from_str(&date_time_sent_f, "%Y-%m-%d").unwrap()));
        }

        let records = query.offset(page).limit(page_size).load::<Notification>(conn)?;
      
        Ok(records)  
    }

    pub fn get_all_user_notifications(id: i32) -> Result<Vec<Notification>, diesel::result::Error> {
        use crate::schema::notifications::dsl::*;
        let conn = &mut establish_connection();

        notifications.filter(id_user.eq(id)).load::<Notification>(conn)
    }

    pub fn insert_notification(notification: NewNotification) -> Result<(), diesel::result::Error> {
        let conn = &mut establish_connection();
    
        let insertable = NewNotificationInsert::from(notification);
        match diesel::insert_into(notifications::table)
            .values(&insertable)
            .execute(conn)
        {
            Ok(_) => Ok(()),
            Err(error) => {
                println!("Error inserting notification: {:?}", error);
                Err(error)
            }
        }
    }

    pub fn update_notification(id: i32, notification: NewNotification) -> Result<(), diesel::result::Error> {
        use crate::schema::notifications::dsl::*;
        let conn = &mut establish_connection();        

        let insertable: NewNotificationInsert = NewNotificationInsert::from(notification);
        match diesel::update(notifications.filter(id_notification.eq(id)))
            .set(&insertable)
            .execute(conn)
        {
            Ok(_) => Ok(()),
            Err(error) => {
                println!("Error updating notification: {:?}", error);
                Err(error)
            }
        }
    }

    pub fn delete_notification(id: i32) -> Result<(), diesel::result::Error> {
        use crate::schema::notifications::dsl::*;
        let conn = &mut establish_connection();

        match diesel::delete(notifications.filter(id_notification.eq(id)))
            .execute(conn)
        {
            Ok(_) => Ok(()),
            Err(error) => {
                println!("Error deleting promotion: {:?}", error);
                Err(error)
            }
        }
    }
}