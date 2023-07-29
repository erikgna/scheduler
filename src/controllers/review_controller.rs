use diesel;
use diesel::prelude::*;
use crate::{schema::reviews, models::review_models::{Review, NewReview, NewReviewInsert}};
use crate::db::establish_connection;

impl Review {
    pub fn get_review(id: i32) -> Result<Review, diesel::result::Error> {
        use crate::schema::reviews::dsl::*;

        let conn = &mut establish_connection();
        reviews.filter(id_review.eq(id)).first::<Review>(conn)        
    }

    pub fn get_all_reviews(
        page_size: i64, 
        page: i64,
        comment_filter: Option<String>,
        rating_filter: Option<String>
    ) -> Result<Vec<Review>, diesel::result::Error> {
        use crate::schema::reviews::dsl::*;
        let conn = &mut establish_connection();

        let mut query = reviews.into_boxed();
        
        if let Some(comment_f) = comment_filter {
            query = query.filter(comment.eq(comment_f));
        }        

        if let Some(rating_f) = rating_filter {            
            query = query.filter(rating.eq(rating_f.parse::<i32>().unwrap_or(1)));
        }

        let records = query.offset(page).limit(page_size).load::<Review>(conn)?;
      
        Ok(records)
    }

    pub fn get_all_user_reviews(id: i32) -> Result<Vec<Review>, diesel::result::Error> {
        use crate::schema::reviews::dsl::*;
        let conn = &mut establish_connection();

        reviews.filter(id_user.eq(id)).load::<Review>(conn)
    }

    pub fn get_all_professional_reviews(id: i32) -> Result<Vec<Review>, diesel::result::Error> {
        use crate::schema::reviews::dsl::*;
        let conn = &mut establish_connection();

        reviews.filter(id_professional.eq(id)).load::<Review>(conn)
    }

    pub fn get_all_service_reviews(id: i32) -> Result<Vec<Review>, diesel::result::Error> {
        use crate::schema::reviews::dsl::*;
        let conn = &mut establish_connection();

        reviews.filter(id_service.eq(id)).load::<Review>(conn)
    }

    pub fn insert_review(review: NewReview) -> Result<(), diesel::result::Error> {
        let conn = &mut establish_connection();
    
        let review_insert = NewReviewInsert::from(review);
        match diesel::insert_into(reviews::table)
            .values(&review_insert)
            .execute(conn)
        {
            Ok(_) => Ok(()),
            Err(error) => {
                println!("Error inserting review: {:?}", error);
                Err(error)
            }
        }
    }

    pub fn update_review(id: i32, review: NewReview) -> Result<(), diesel::result::Error> {
        use crate::schema::reviews::dsl::*;
        let conn = &mut establish_connection();        

        let review_insert = NewReviewInsert::from(review);
        match diesel::update(reviews.filter(id_review.eq(id)))
            .set(&review_insert)
            .execute(conn)
        {
            Ok(_) => Ok(()),
            Err(error) => {
                println!("Error updating review: {:?}", error);
                Err(error)
            }
        }
    }

    pub fn delete_review(id: i32) -> Result<(), diesel::result::Error> {
        use crate::schema::reviews::dsl::*;
        let conn = &mut establish_connection();

        match diesel::delete(reviews.filter(id_review.eq(id)))
            .execute(conn)
        {
            Ok(_) => Ok(()),
            Err(error) => {
                println!("Error deleting review: {:?}", error);
                Err(error)
            }
        }
    }
}