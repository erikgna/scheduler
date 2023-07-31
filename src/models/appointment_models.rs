use chrono::NaiveDateTime;
use crate::schema::appointments;

#[derive(Queryable, Serialize)]
pub struct Appointment {
    pub id_appointment: i32,
    pub id_user: Option<i32>,
    pub id_professional: Option<i32>,
    pub id_service: Option<i32>,
    pub date_time_appointment: NaiveDateTime,
    pub appointment_status: String
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct NewAppointment {    
    pub id_user: Option<i32>,
    pub id_professional: Option<i32>,
    pub id_service: Option<i32>,
    pub date_time_appointment: NaiveDateTime,
    pub appointment_status: String
}

#[derive(Insertable, PartialEq, Eq, Debug, Clone, AsChangeset)]
#[diesel(table_name = appointments)]
pub struct NewAppointmentInsert {        
    pub id_user: Option<i32>,
    pub id_professional: Option<i32>,
    pub id_service: Option<i32>,
    pub date_time_appointment: NaiveDateTime,
    pub appointment_status: String
}

impl From<NewAppointment> for NewAppointmentInsert {
    fn from(appointment: NewAppointment) -> Self {
        NewAppointmentInsert {
            id_user: appointment.id_user,
            id_professional: appointment.id_professional,
            id_service: appointment.id_service,
            date_time_appointment: appointment.date_time_appointment,
            appointment_status: appointment.appointment_status
        }
    }
}