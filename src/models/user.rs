use super::schema::{user, user_club_relation};
use diesel::Queryable;

#[derive(Debug)]
pub enum UserPermission {
    NewcomerOrNone = 0,
    ClubMember = 1,
    Moderator = 2,
    Admin = 3,
}

#[derive(Debug, Queryable, Identifiable)]
#[table_name = "user"]
pub struct User {
    pub id: u64,
    pub name: String,
    pub icon: String,
    pub screen_name: String,
    pub permission: UserPermission,
    pub access_token: String,
    pub access_token_secret: String,
}

#[derive(Debug, Queryable, Identifiable)]
#[table_name = "user_club_relation"]
#[primary_key(user_id, club_id)]
pub struct UserClubRelation {
    pub user_id: u64,
    pub club_id: u64,
    pub level: u32,
}
