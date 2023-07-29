use chrono::NaiveDateTime;
use diesel;
use diesel::prelude::*;
use crate::{schema::appointments, models::appointment_models::{Appointment, NewAppointment, NewAppointmentInsert}};
use crate::db::establish_connection;

impl Appointment {
    pub fn get_appointment(id: i32) -> Result<Appointment, diesel::result::Error> {
        use crate::schema::appointments::dsl::*;

        let conn = &mut establish_connection();
        appointments.filter(id_appointment.eq(id)).first::<Appointment>(conn)        
    }

    pub fn get_all_appointments(
        page_size: i64, 
        page: i64,
        date_time_appointment_filter: Option<String>,
        appointment_status_filter: Option<String>
    ) -> Result<Vec<Appointment>, diesel::result::Error> {
        use crate::schema::appointments::dsl::*;
        let conn = &mut establish_connection();

        let mut query = appointments.into_boxed();
        
        if let Some(date_time_appointment_f) = date_time_appointment_filter {
            query = query.filter(date_time_appointment.le(NaiveDateTime::parse_from_str(&date_time_appointment_f, "%Y-%m-%d").unwrap()));            
        }        

        if let Some(appointment_status_f) = appointment_status_filter {            
            query = query.filter(appointment_status.eq(appointment_status_f));
        }

        let records = query.offset(page).limit(page_size).load::<Appointment>(conn)?;
      
        Ok(records)  
    }

    pub fn get_all_user_appointments(id: i32) -> Result<Vec<Appointment>, diesel::result::Error> {
        use crate::schema::appointments::dsl::*;
        let conn = &mut establish_connection();

        appointments.filter(id_user.eq(id)).load::<Appointment>(conn)
    }

    pub fn get_all_professional_appointments(id: i32) -> Result<Vec<Appointment>, diesel::result::Error> {
        use crate::schema::appointments::dsl::*;
        let conn = &mut establish_connection();

        appointments.filter(id_professional.eq(id)).load::<Appointment>(conn)
    }

    pub fn get_all_service_appointments(id: i32) -> Result<Vec<Appointment>, diesel::result::Error> {
        use crate::schema::appointments::dsl::*;
        let conn = &mut establish_connection();

        appointments.filter(id_service.eq(id)).load::<Appointment>(conn)
    }

    pub fn insert_appointment(appointment: NewAppointment) -> Result<(), diesel::result::Error> {
        let conn = &mut establish_connection();
    
        let insertable = NewAppointmentInsert::from(appointment);
        match diesel::insert_into(appointments::table)
            .values(&insertable)
            .execute(conn)
        {
            Ok(_) => Ok(()),
            Err(error) => {
                println!("Error inserting appointment: {:?}", error);
                Err(error)
            }
        }
    }

    pub fn update_appointment(id: i32, appointment: NewAppointment) -> Result<(), diesel::result::Error> {
        use crate::schema::appointments::dsl::*;
        let conn = &mut establish_connection();        

        let insertable = NewAppointmentInsert::from(appointment);
        match diesel::update(appointments.filter(id_appointment.eq(id)))
            .set(&insertable)
            .execute(conn)
        {
            Ok(_) => Ok(()),
            Err(error) => {
                println!("Error updating appointment: {:?}", error);
                Err(error)
            }
        }
    }

    pub fn delete_appointment(id: i32) -> Result<(), diesel::result::Error> {
        use crate::schema::appointments::dsl::*;
        let conn = &mut establish_connection();

        match diesel::delete(appointments.filter(id_appointment.eq(id)))
            .execute(conn)
        {
            Ok(_) => Ok(()),
            Err(error) => {
                println!("Error deleting appointment: {:?}", error);
                Err(error)
            }
        }
    }
}