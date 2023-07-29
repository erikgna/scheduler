use bigdecimal::BigDecimal;
use diesel::prelude::*;
use crate::{schema::services, models::service_models::{Service, NewServiceInsert, NewService}};
use crate::db::establish_connection;

impl Service {
    pub fn insert_service(service: NewService) -> Result<(), diesel::result::Error> {
        let conn = &mut establish_connection();
    
        let service_insert = NewServiceInsert::from(service);
        match diesel::insert_into(services::table)
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

    pub fn get_service(id: i32) -> Result<Service, diesel::result::Error> {
        use crate::schema::services::dsl::*;

        let conn = &mut establish_connection();
        services.filter(id_service.eq(id)).first::<Service>(conn)        
    }

    pub fn get_all_services(
        page_size: i64, 
        page: i64,
        service_name_filter: Option<String>,
        price_filter: Option<String>,
        duration_filter: Option<String>
    ) -> Result<Vec<Service>, diesel::result::Error> {
        use crate::schema::services::dsl::*;
        let conn = &mut establish_connection();

        let mut query = services.into_boxed();
        
        if let Some(service_name_f) = service_name_filter {
            query = query.filter(service_name.eq(service_name_f));            
        }        

        if let Some(price_f) = price_filter {            
            query = query.filter(price.eq(price_f.parse::<BigDecimal>().unwrap()));
        }

        if let Some(duration_f) = duration_filter {            
            query = query.filter(duration.eq(duration_f.parse::<i32>().unwrap_or(1)));
        }

        let records = query.offset(page).limit(page_size).load::<Service>(conn)?;
      
        Ok(records)
    }

    pub fn get_all_professional_services(id: i32) -> Result<Vec<Service>, diesel::result::Error> {
        use crate::schema::services::dsl::*;
        let conn = &mut establish_connection();

        services.filter(id_professional.eq(id)).load::<Service>(conn)
    }

    pub fn update_service(id: i32, service: NewService) -> Result<(), diesel::result::Error> {
        use crate::schema::services::dsl::*;
        let conn = &mut establish_connection();        

        let service_insert = NewServiceInsert::from(service);
        match diesel::update(services.filter(id_service.eq(id)))
            .set(&service_insert)
            .execute(conn)
        {
            Ok(_) => Ok(()),
            Err(error) => {
                println!("Error updating service: {:?}", error);
                Err(error)
            }
        }
    }

    pub fn delete_service(id: i32) -> Result<(), diesel::result::Error> {
        use crate::schema::services::dsl::*;
        let conn = &mut establish_connection();

        match diesel::delete(services.filter(id_service.eq(id)))
            .execute(conn)
        {
            Ok(_) => Ok(()),
            Err(error) => {
                println!("Error deleting service: {:?}", error);
                Err(error)
            }
        }
    }

    pub fn change_photo(photo_path: String, id: i32) -> Result<(), diesel::result::Error>{
        let conn = &mut establish_connection();
        
        conn.transaction(|conn| {
            diesel::update(services::table.filter(services::id_service.eq(id)))
                .set(services::images.eq(photo_path))
                .execute(conn)?;
            
            Ok(())
        })
    }    
}