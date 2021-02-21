use crate::{models::schema::asset, utils::StringNumber};
use async_graphql::InputObject;

#[derive(Insertable)]
#[table_name = "asset"]
pub struct NewAssetDto {
    pub owner_id: StringNumber,
    pub club_id: String,
    pub name: String,
    pub file_path: String,
    pub file_size: i32,
}

#[derive(Debug, AsChangeset, InputObject)]
#[table_name = "asset"]
pub struct UpdateAssetDto {
    pub alternative_description: Option<String>,
}
