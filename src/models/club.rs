use super::schema::club;
use diesel::{Identifiable, Queryable};

#[derive(Debug, Queryable, Identifiable, Associations)]
#[table_name = "club"]
#[belongs_to(super::UserClubRelation, foreign_key = "id")]
pub struct Club {
    pub id: String,
    pub name: String,
    pub is_published: bool,
    pub short_description: Option<String>,
    pub long_description: Option<String>,
    pub join_description: Option<String>,
    pub place: Option<String>,
    pub schedule: Option<String>,
    pub video_url: Option<String>,
    pub contact_url: Option<String>,
}
