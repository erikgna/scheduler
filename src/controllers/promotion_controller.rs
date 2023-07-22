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

    pub fn get_all_promotions() -> Result<Vec<Promotion>, diesel::result::Error> {
        use crate::schema::promotions::dsl::*;
        let conn = &mut establish_connection();

        promotions.load::<Promotion>(conn)
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