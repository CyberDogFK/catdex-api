use diesel::{Queryable, Insertable};
use serde::Serialize;
use crate::schema::cats;

#[derive(Queryable, Serialize)]
pub struct Cat {
    pub id: i32,
    pub name: String,
    pub image_path: String,
}

#[derive(Insertable, Serialize)]
#[diesel(table_name = cats)]
pub struct NewCat {
    // id will be added by the database
    pub name: String,
    pub image_path: String,
}