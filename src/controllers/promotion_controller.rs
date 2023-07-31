use chrono::NaiveDate;
use diesel;
use diesel::prelude::*;
use crate::{schema::promotions, models::promotion_models::{Promotion, NewPromotion, NewPromotionInsert}};
use crate::db::establish_connection;

impl Promotion {
    pub fn get_promotion(id: i32) -> Result<Promotion, diesel::result::Error> {
        use crate::schema::promotions::dsl::*;

        let conn = &mut establish_connection();
        promotions.filter(id_promotion.eq(id)).first::<Promotion>(conn)        
    }

    pub fn get_all_promotions(
        page_size: i64, 
        page: i64,
        name: Option<String>,
        start_date_param: Option<String>,
        end_date_param: Option<String>,
    ) -> Result<Vec<Promotion>, diesel::result::Error> {
        use crate::schema::promotions::dsl::*;

        let conn = &mut establish_connection();        
        let mut query = promotions.into_boxed();
        
        if let Some(name_filter) = name {
            query = query.filter(promotion_name.eq(name_filter));
        }        

        if let Some(start_date_filter) = start_date_param {            
            query = query.filter(start_date.le(NaiveDate::parse_from_str(&start_date_filter, "%Y-%m-%d").unwrap()));
        }

        if let Some(end_date_filter) = end_date_param {            
            query = query.filter(end_date.le(NaiveDate::parse_from_str(&end_date_filter, "%Y-%m-%d").unwrap()));
        }
        
        let records = query.offset(page).limit(page_size).load::<Promotion>(conn)?;
      
        Ok(records)
    }

    pub fn insert_promotion(promotion: NewPromotion) -> Result<(), diesel::result::Error> {
        let conn = &mut establish_connection();
    
        let insertable = NewPromotionInsert::from(promotion);
        match diesel::insert_into(promotions::table)
            .values(&insertable)
            .execute(conn)
        {
            Ok(_) => Ok(()),
            Err(error) => {
                println!("Error inserting promotion: {:?}", error);
                Err(error)
            }
        }
    }

    pub fn update_promotion(id: i32, promotion: NewPromotion) -> Result<(), diesel::result::Error> {
        use crate::schema::promotions::dsl::*;
        let conn = &mut establish_connection();        

        let insertable = NewPromotionInsert::from(promotion);
        match diesel::update(promotions.filter(id_promotion.eq(id)))
            .set(&insertable)
            .execute(conn)
        {
            Ok(_) => Ok(()),
            Err(error) => {
                println!("Error updating promotion: {:?}", error);
                Err(error)
            }
        }
    }

    pub fn delete_promotion(id: i32) -> Result<(), diesel::result::Error> {
        use crate::schema::promotions::dsl::*;
        let conn = &mut establish_connection();

        match diesel::delete(promotions.filter(id_promotion.eq(id)))
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