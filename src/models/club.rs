use super::schema::club;
use async_graphql::SimpleObject;
use diesel::{Identifiable, Queryable};

#[derive(Debug, Queryable, Identifiable, SimpleObject)]
#[table_name = "club"]
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
