use crate::schema::professionals;

#[derive(Queryable, Serialize)]
pub struct Professional {
    pub id: i32,
    pub name: String,
    pub specialization: String,
    pub description: Option<String>,
    pub schedules: Option<String>,
    pub photo_path: Option<String>,
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct NewProfessional {
    pub name: String,
    pub specialization: String,
    pub description: Option<String>,
    pub schedules: Option<String>,
    pub photo_path: Option<String>,
}

#[derive(Insertable, PartialEq, Eq, Debug, Clone, AsChangeset)]
#[table_name = "professionals"]
pub struct NewProfessionalInsert {    
    pub name: String,
    pub specialization: String,
    pub description: Option<String>,
    pub schedules: Option<String>,
    pub photo_path: Option<String>,
}

// Implement a conversion function from `NewUser` to `NewUserInsert`
impl From<NewProfessional> for NewProfessionalInsert {
    fn from(professional: NewProfessional) -> Self {
        NewProfessionalInsert {
            name: professional.name,
            specialization: professional.specialization,
            description: professional.description,
            schedules: professional.schedules,
            photo_path: professional.photo_path,
        }
    }
}
