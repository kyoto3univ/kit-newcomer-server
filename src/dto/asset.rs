use crate::{models::schema::asset, utils::StringNumber};

#[derive(Insertable)]
#[table_name = "asset"]
pub struct NewAssetDto {
    pub owner_id: StringNumber,
    pub club_id: String,
    pub name: String,
    pub file_path: String,
    pub file_size: i32,
}
