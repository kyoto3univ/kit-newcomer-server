use crate::define_enum;

use super::schema::club;
use async_graphql::Enum;
use chrono::NaiveDateTime;
use diesel::{Identifiable, Queryable};

define_enum! {
    #[derive(Debug, Clone, Copy, Eq, PartialEq, PartialOrd, Enum)]
    pub enum ClubTopImageType {
        Image = 0,
        YouTube = 1,
    }
}

#[derive(Debug, Clone, Queryable, Identifiable, Associations)]
#[table_name = "club"]
#[belongs_to(super::UserClubRelation, foreign_key = "id")]
pub struct Club {
    pub id: String,
    pub name: String,
    pub is_published: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub short_description: Option<String>,
    pub long_description: Option<String>,
    pub join_description: Option<String>,
    pub top_image_id: Option<i64>,
    pub top_content_type: ClubTopImageType,
    pub thumb_image_id: Option<i64>,
    pub place: Option<String>,
    pub schedule: Option<String>,
    pub video_url: Option<String>,
    pub contact_url: Option<String>,
}
