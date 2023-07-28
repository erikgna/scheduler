use crate::schema::professionals;

#[derive(Queryable, Serialize)]
pub struct Professional {
    pub id: i32,
    pub id_user: i32,    
    pub specialization: String,
    pub description: Option<String>,
    pub schedules: Option<String>,    
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct NewProfessional {
    pub id_user: i32,    
    pub specialization: String,
    pub description: Option<String>,
    pub schedules: Option<String>,    
}

#[derive(Insertable, PartialEq, Eq, Debug, Clone, AsChangeset)]
#[diesel(table_name = professionals)]
pub struct NewProfessionalInsert {    
    pub id_user: i32,    
    pub specialization: String,
    pub description: Option<String>,
    pub schedules: Option<String>,    
}

impl From<NewProfessional> for NewProfessionalInsert {
    fn from(professional: NewProfessional) -> Self {
        NewProfessionalInsert {
            id_user: professional.id_user,
            specialization: professional.specialization,
            description: professional.description,
            schedules: professional.schedules            
        }
    }
}
