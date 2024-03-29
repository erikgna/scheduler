use diesel;
use diesel::prelude::*;
use crate::schema::professionals;

use crate::models::professional_models::{NewProfessional, NewProfessionalInsert};
use crate::{models::professional_models::Professional, db::establish_connection};

impl Professional{
    pub fn insert_professional(professional: NewProfessional, id: i32) -> Result<(), diesel::result::Error>{
        let conn = &mut establish_connection();
        
        let mut professional_insert = NewProfessionalInsert::from(professional);                
        professional_insert.id_user = id;

        match diesel::insert_into(professionals::table)
            .values(&professional_insert)
            .execute(conn)
        {
            Ok(_) => Ok(()),
            Err(error) => {
                println!("Error inserting professional: {:?}", error);
                Err(error)
            }
        }
    }
    
    pub fn get_user_professional(user_id: i32) -> Result<Professional, diesel::result::Error> {
        use crate::schema::professionals::dsl::*;

        let conn = &mut establish_connection();        
        professionals.filter(id_user.eq(user_id)).first::<Professional>(conn)        
    }

    pub fn get_professional(id: i32) -> Result<Professional, diesel::result::Error> {
        use crate::schema::professionals::dsl::*;

        let conn = &mut establish_connection();        
        professionals.filter(id_professional.eq(id)).first::<Professional>(conn)        
    }

    pub fn get_all_professionals(
        page: i64,
        page_size: i64,
        specialization_filter: Option<String>,
    ) -> Result<Vec<Professional>, diesel::result::Error> {
        use crate::schema::professionals::dsl::*;

        let conn = &mut establish_connection();        
        let mut query = professionals.into_boxed();
        
        if let Some(spec_filter) = specialization_filter {
            query = query.filter(specialization.eq(spec_filter));
        }        

        let records = query.offset(page).limit(page_size).load::<Professional>(conn)?;
      
        Ok(records)  
    }

    pub fn update_professional(id: i32, professional: NewProfessional) -> Result<(), diesel::result::Error> {
        use crate::schema::professionals::dsl::*;
        let conn = &mut establish_connection();

        let professional_insert = NewProfessionalInsert::from(professional);

        match diesel::update(professionals.filter(id_professional.eq(id)))
            .set(&professional_insert)
            .execute(conn)
        {
            Ok(_) => Ok(()),
            Err(error) => {
                println!("Error updating professional: {:?}", error);
                Err(error)
            }
        }
    }

    pub fn delete_professional(id: i32) -> Result<(), diesel::result::Error> {
        use crate::schema::professionals::dsl::*;
        let conn = &mut establish_connection();

        match diesel::delete(professionals.filter(id_professional.eq(id)))
            .execute(conn)
        {
            Ok(_) => Ok(()),
            Err(error) => {
                println!("Error deleting professional: {:?}", error);
                Err(error)
            }
        }
    }
}