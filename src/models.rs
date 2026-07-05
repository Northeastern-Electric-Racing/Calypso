use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::test_profile)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct TestProfile {
    pub id: i32,
    pub name: String,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::can_message)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct TestCanMessageEntry {
    pub id: i32,
    pub profile_id: i32,
    pub can_id: i32,
    pub is_extended: i32,
    pub data: Vec<u8>,
    pub mode: String,
    pub offset_ms: i32,
    pub period_ms: Option<i32>,
}

