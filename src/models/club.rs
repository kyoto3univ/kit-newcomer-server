use diesel::Queryable;

#[derive(Debug, Queryable)]
pub struct Club {
    pub id: String,
    pub name: String,
    pub is_published: bool,
    pub short_description: String,
    pub long_description: String,
    pub join_description: String,
    pub place: String,
    pub schedule: String,
    pub video_url: String,
    pub contact_url: String,
}
