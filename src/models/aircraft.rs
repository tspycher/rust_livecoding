use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Queryable, Serialize, Selectable, ToSchema, Debug, Clone)]
#[diesel(table_name = crate::schema::aircraft)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Aircraft {
    pub id: Option<i32>,
    pub display_name: String,
    pub registration: String,
}

// Define NewAircraft struct for inserting new records
#[derive(Insertable, Deserialize, ToSchema, Debug, Clone)]
#[diesel(table_name = crate::schema::aircraft)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct NewAircraft {
    display_name: String,
    registration: String,
}
