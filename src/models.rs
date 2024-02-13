use diesel::Queryable;
use serde::Serialize;

#[derive(Queryable, Serialize)]
pub struct Cat {
    pub id: i32,
    pub name: String,
    pub image_path: String,
}
