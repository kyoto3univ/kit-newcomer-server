use async_graphql::InputObject;
use diesel::{prelude::*, r2d2::ConnectionManager};
use r2d2::PooledConnection;

use crate::{
    models::{schema::club, Asset, ClubTopImageType},
    utils::StringNumber,
};

#[derive(Debug, Insertable)]
#[table_name = "club"]
pub struct NewClubDto<'a> {
    pub id: &'a str,
    pub name: &'a str,
    pub is_published: bool,
}

#[derive(Debug, Clone, InputObject, AsChangeset)]
#[table_name = "club"]
pub struct UpdateClubDto {
    pub short_description: Option<String>,
    pub long_description: Option<String>,
    pub join_description: Option<String>,
    pub place: Option<String>,
    pub schedule: Option<String>,
    pub video_url: Option<String>,
    pub contact_url: Option<String>,
    pub top_content_type: Option<ClubTopImageType>,
}

#[derive(Debug, Clone, InputObject, AsChangeset)]
#[table_name = "club"]
pub struct UpdateClubAssetDto {
    pub top_image_id: Option<StringNumber>,
    pub thumb_image_id: Option<StringNumber>,
}

impl UpdateClubAssetDto {
    pub fn validate(
        &self,
        conn: &PooledConnection<ConnectionManager<MysqlConnection>>,
        club_id: &String,
    ) -> Result<bool, anyhow::Error> {
        if let Some(top_image_id) = &self.top_image_id {
            if !Asset::check_permission(conn, top_image_id, club_id)? {
                return Ok(false);
            }
        }
        if let Some(thumb_image_id) = &self.thumb_image_id {
            if !Asset::check_permission(conn, thumb_image_id, club_id)? {
                return Ok(false);
            }
        }
        Ok(true)
    }
}
