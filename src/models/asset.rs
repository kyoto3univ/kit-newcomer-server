use super::schema::asset;
use diesel::{Identifiable, Queryable};

#[derive(Debug, Queryable, Associations, Identifiable)]
#[belongs_to(super::Club, foreign_key = "club_id")]
#[table_name = "asset"]
pub struct Asset {
    pub id: u64,
    pub owner_id: u64,
    pub club_id: u64,
    pub name: String,
    pub alternative_description: String,
    pub file_path: String,
    pub file_size: u32,
    pub image_width: u32,
    pub image_height: u32,
}
