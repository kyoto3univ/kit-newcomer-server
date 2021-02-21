use async_graphql::InputObject;

use crate::models::{schema::club, ClubTopImageType};

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
