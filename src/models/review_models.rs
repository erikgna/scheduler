use crate::schema::reviews;

#[derive(Queryable, Serialize)]
pub struct Review {
    pub id_review: i32,
    pub id_user: Option<i32>,
    pub id_professional: Option<i32>,
    pub comment: Option<String>,
    pub rating: Option<i32>,
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct NewReview {    
    pub id_user: Option<i32>,
    pub id_professional: Option<i32>,
    pub comment: Option<String>,
    pub rating: Option<i32>,    
}

#[derive(Insertable, PartialEq, Eq, Debug, Clone, AsChangeset)]
#[diesel(table_name = reviews)]
pub struct NewReviewInsert {        
    pub id_user: Option<i32>,
    pub id_professional: Option<i32>,
    pub comment: Option<String>,
    pub rating: Option<i32>,
}

impl From<NewReview> for NewReviewInsert {
    fn from(review: NewReview) -> Self {
        NewReviewInsert {
            id_user: review.id_user,
            id_professional: review.id_professional,
            comment: review.comment,
            rating: review.rating,            
        }
    }
}