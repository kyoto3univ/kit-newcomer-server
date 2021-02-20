use super::schema::asset;
use crate::utils::StringNumber;
use async_graphql::SimpleObject;
use diesel::{Identifiable, Queryable};

#[derive(Debug, Queryable, Associations, Identifiable, Insertable, SimpleObject)]
#[belongs_to(super::Club, foreign_key = "club_id")]
#[belongs_to(super::User, foreign_key = "owner_id")]
#[table_name = "asset"]
pub struct Asset {
    pub id: StringNumber,
    pub owner_id: StringNumber,
    pub club_id: String,
    pub name: String,
    pub alternative_description: Option<String>,
    pub file_path: String,
    pub file_size: i32,
    pub image_width: Option<i32>,
    pub image_height: Option<i32>,
}
