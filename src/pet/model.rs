use diesel::{Queryable, Selectable};

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::pets)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Pet{
    id: i32,
    name: String,
    category_id: i32,
    status: String,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::categories)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Category {
    id: i32,
    name: String,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::tags)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Tag {
    id: i32,
    name: String,
}