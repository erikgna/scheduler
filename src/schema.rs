// @generated automatically by Diesel CLI.

diesel::table! {
    appointments (id_appointment) {
        id_appointment -> Int4,
        id_user -> Nullable<Int4>,
        id_professional -> Nullable<Int4>,
        id_service -> Nullable<Int4>,
        date_time_appointment -> Timestamp,
        appointment_status -> Varchar,
    }
}

diesel::table! {
    notifications (id_notification) {
        id_notification -> Int4,
        id_user -> Nullable<Int4>,
        message -> Text,
        date_time_sent -> Timestamp,
    }
}

diesel::table! {
    professionals (id_professional) {
        id_professional -> Int4,
        name -> Varchar,
        specialization -> Varchar,
        description -> Nullable<Text>,
        schedules -> Nullable<Text>,
        photo_path -> Nullable<Varchar>,
    }
}

diesel::table! {
    promotions (id_promotion) {
        id_promotion -> Int4,
        promotion_name -> Varchar,
        description -> Nullable<Text>,
        start_date -> Date,
        end_date -> Date,
        discount_code -> Nullable<Varchar>,
    }
}

diesel::table! {
    reviews (id_review) {
        id_review -> Int4,
        id_user -> Nullable<Int4>,
        id_professional -> Nullable<Int4>,
        comment -> Nullable<Text>,
        rating -> Nullable<Int4>,
    }
}

diesel::table! {
    service_history (id_record) {
        id_record -> Int4,
        id_user -> Nullable<Int4>,
        id_service -> Nullable<Int4>,
        date_time_service -> Timestamp,
        amount_paid -> Numeric,
    }
}

diesel::table! {
    services (id_service) {
        id_service -> Int4,
        service_name -> Varchar,
        description -> Nullable<Text>,
        images -> Nullable<Text>,
        price -> Numeric,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        email -> Varchar,
        first_name -> Varchar,
        last_name -> Varchar,
        photo -> Varchar,
        phone -> Varchar,
        password -> Varchar,
        token -> Nullable<Varchar>,
    }
}

diesel::joinable!(appointments -> professionals (id_professional));
diesel::joinable!(appointments -> services (id_service));
diesel::joinable!(appointments -> users (id_user));
diesel::joinable!(notifications -> users (id_user));
diesel::joinable!(reviews -> professionals (id_professional));
diesel::joinable!(reviews -> users (id_user));
diesel::joinable!(service_history -> services (id_service));
diesel::joinable!(service_history -> users (id_user));

diesel::allow_tables_to_appear_in_same_query!(
    appointments,
    notifications,
    professionals,
    promotions,
    reviews,
    service_history,
    services,
    users,
);
