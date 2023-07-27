use diesel;
use diesel::prelude::*;
use crate::schema::professionals;

use crate::models::professional_models::{NewProfessional, NewProfessionalInsert};
use crate::{models::professional_models::Professional, db::establish_connection};

impl Professional{
    pub fn insert_professional(professional: NewProfessional) -> Result<(), diesel::result::Error>{
        let conn = &mut establish_connection();
        
        let professional_insert = NewProfessionalInsert::from(professional);

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
    
    pub fn get_professional(id: i32) -> Result<Professional, diesel::result::Error> {
        use crate::schema::professionals::dsl::*;

        let conn = &mut establish_connection();        
        professionals.filter(id_professional.eq(id)).first::<Professional>(conn)        
    }

    pub fn get_all_professionals() -> Result<Vec<Professional>, diesel::result::Error> {
        use crate::schema::professionals::dsl::*;
        let conn = &mut establish_connection();

        professionals.load::<Professional>(conn)
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

    pub fn change_photo(photo_path: String, id: i32) -> Result<(), diesel::result::Error>{
        let conn = &mut establish_connection();
        
        conn.transaction(|conn| {
            diesel::update(professionals::table.filter(professionals::id_professional.eq(id)))
                .set(professionals::photo_path.eq(photo_path))
                .execute(conn)?;
            
            Ok(())
        })
    }      

    pub fn delete_photo(id: i32) -> Result<(), diesel::result::Error>{
        let conn = &mut establish_connection();
        
        conn.transaction(|conn| {
            diesel::update(professionals::table.filter(professionals::id_professional.eq(id)))
                .set(professionals::photo_path.eq(""))
                .execute(conn)?;
            
            Ok(())
        })
    } 
}