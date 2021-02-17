use crate::define_enum;

use super::schema::{user, user_club_relation};
use async_graphql::{Enum, SimpleObject};
use diesel::Queryable;

define_enum! {
    #[derive(Debug, Clone, Copy, Eq, PartialEq, Enum)]
    pub enum UserPermission {
        NewcomerOrNone = 0,
        ClubMember = 1,
        Moderator = 2,
        Admin = 3,
    }
}

#[derive(Debug, Queryable, Identifiable, Insertable, SimpleObject)]
#[table_name = "user"]
pub struct User {
    pub id: i64,
    pub name: String,
    pub screen_name: String,
    pub icon: Option<String>,
    pub permission: UserPermission,
    #[graphql(skip)]
    pub access_token: Option<String>,
    #[graphql(skip)]
    pub access_token_secret: Option<String>,
}

#[derive(Debug, Queryable, Identifiable, Insertable)]
#[table_name = "user_club_relation"]
#[primary_key(user_id, club_id)]
pub struct UserClubRelation {
    pub user_id: i64,
    pub club_id: i64,
    pub level: i32,
}
