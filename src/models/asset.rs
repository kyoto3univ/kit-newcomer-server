use super::schema::asset;
use async_graphql::SimpleObject;
use diesel::{Identifiable, Queryable};

#[derive(Debug, Queryable, Associations, Identifiable, Insertable, SimpleObject)]
#[belongs_to(super::Club, foreign_key = "club_id")]
#[table_name = "asset"]
pub struct Asset {
    pub id: i64,
    pub owner_id: i64,
    pub club_id: i64,
    pub name: String,
    pub alternative_description: Option<String>,
    pub file_path: String,
    pub file_size: i32,
    pub image_width: Option<i32>,
    pub image_height: Option<i32>,
}
