use diesel;
use diesel::prelude::*;
use crate::db::establish_connection;
use crate::schema::service_history;
use crate::models::service_history_models::{NewServiceHistoryInsert, NewServiceHistory, ServiceHistory};

impl ServiceHistory {
    pub fn insert_service_history(service: NewServiceHistory) -> Result<(), diesel::result::Error> {
        let conn = &mut establish_connection();
    
        let service_insert = NewServiceHistoryInsert::from(service);
        match diesel::insert_into(service_history::table)
            .values(&service_insert)
            .execute(conn)
        {
            Ok(_) => Ok(()),
            Err(error) => {
                println!("Error inserting service: {:?}", error);
                Err(error)
            }
        }
    }

    pub fn get_service_history(id: i32) -> Result<ServiceHistory, diesel::result::Error> {
        use crate::schema::service_history::dsl::*;

        let conn = &mut establish_connection();
        service_history.filter(id_service.eq(id)).first::<ServiceHistory>(conn)        
    }

    pub fn get_all_services_history() -> Result<Vec<ServiceHistory>, diesel::result::Error> {
        use crate::schema::service_history::dsl::*;
        let conn = &mut establish_connection();

        service_history.load::<ServiceHistory>(conn)
    }

    pub fn get_all_user_service_history(id: i32) -> Result<Vec<ServiceHistory>, diesel::result::Error> {
        use crate::schema::service_history::dsl::*;
        let conn = &mut establish_connection();

        service_history.filter(id_user.eq(id)).load::<ServiceHistory>(conn)
    }

    pub fn update_service_history(id: i32, service: NewServiceHistory) -> Result<(), diesel::result::Error> {
        use crate::schema::service_history::dsl::*;
        let conn = &mut establish_connection();        

        let service_insert = NewServiceHistoryInsert::from(service);
        match diesel::update(service_history.filter(id_service.eq(id)))
            .set(&service_insert)
            .execute(conn)
        {
            Ok(_) => Ok(()),
            Err(error) => {
                println!("Error updating service history: {:?}", error);
                Err(error)
            }
        }
    }

    pub fn delete_service_history(id: i32) -> Result<(), diesel::result::Error> {
        use crate::schema::service_history::dsl::*;
        let conn = &mut establish_connection();

        match diesel::delete(service_history.filter(id_service.eq(id)))
            .execute(conn)
        {
            Ok(_) => Ok(()),
            Err(error) => {
                println!("Error deleting service history: {:?}", error);
                Err(error)
            }
        }
    }
}