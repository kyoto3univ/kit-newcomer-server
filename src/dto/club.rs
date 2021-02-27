use async_graphql::{InputObject, MaybeUndefined};
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

#[derive(Debug, Clone, InputObject)]
pub struct UpdateClubDto {
    pub name: Option<String>,
    pub short_description: MaybeUndefined<String>,
    pub long_description: MaybeUndefined<String>,
    pub join_description: MaybeUndefined<String>,
    pub place: MaybeUndefined<String>,
    pub schedule: MaybeUndefined<String>,
    pub video_url: MaybeUndefined<String>,
    pub contact_url: MaybeUndefined<String>,
    pub top_content_type: Option<ClubTopImageType>,
    pub top_image_id: MaybeUndefined<StringNumber>,
    pub thumb_image_id: MaybeUndefined<StringNumber>,
}

impl UpdateClubDto {
    pub fn validate(
        &self,
        conn: &PooledConnection<ConnectionManager<MysqlConnection>>,
        club_id: &String,
    ) -> Result<bool, anyhow::Error> {
        if let MaybeUndefined::Value(top_image_id) = &self.top_image_id {
            if !Asset::check_permission(conn, top_image_id, club_id)? {
                return Ok(false);
            }
        }
        if let MaybeUndefined::Value(thumb_image_id) = &self.thumb_image_id {
            if !Asset::check_permission(conn, thumb_image_id, club_id)? {
                return Ok(false);
            }
        }
        Ok(true)
    }
}

#[derive(Debug, Clone, AsChangeset)]
#[table_name = "club"]
pub struct UpdateClubDbDto {
    pub name: Option<String>,
    pub short_description: Option<Option<String>>,
    pub long_description: Option<Option<String>>,
    pub join_description: Option<Option<String>>,
    pub place: Option<Option<String>>,
    pub schedule: Option<Option<String>>,
    pub video_url: Option<Option<String>>,
    pub contact_url: Option<Option<String>>,
    pub top_content_type: Option<ClubTopImageType>,
    pub top_image_id: Option<Option<StringNumber>>,
    pub thumb_image_id: Option<Option<StringNumber>>,
}

fn maybeundef_to_option<T>(val: MaybeUndefined<T>) -> Option<Option<T>> {
    match val {
        MaybeUndefined::Undefined => None,
        MaybeUndefined::Null => Some(None),
        MaybeUndefined::Value(v) => Some(Some(v)),
    }
}

impl From<UpdateClubDto> for UpdateClubDbDto {
    fn from(v: UpdateClubDto) -> Self {
        Self {
            name: v.name,
            short_description: maybeundef_to_option(v.short_description),
            long_description: maybeundef_to_option(v.long_description),
            join_description: maybeundef_to_option(v.join_description),
            place: maybeundef_to_option(v.place),
            schedule: maybeundef_to_option(v.schedule),
            video_url: maybeundef_to_option(v.video_url),
            contact_url: maybeundef_to_option(v.contact_url),
            top_content_type: v.top_content_type,
            top_image_id: maybeundef_to_option(v.top_image_id),
            thumb_image_id: maybeundef_to_option(v.thumb_image_id),
        }
    }
}
