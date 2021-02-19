use crate::{define_enum, utils::StringNumber};

use super::schema::{user, user_club_relation};
use async_graphql::{Enum, SimpleObject};
use diesel::Queryable;

define_enum! {
    #[derive(Debug, Clone, Copy, Eq, PartialEq, PartialOrd, Enum)]
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
    pub id: StringNumber,
    pub name: String,
    pub screen_name: String,
    pub icon: Option<String>,
    pub permission: UserPermission,
    #[graphql(skip)]
    pub access_token: Option<String>,
    #[graphql(skip)]
    pub access_token_secret: Option<String>,
}

define_enum! {
    #[derive(Debug, Clone, Copy, Eq, PartialEq, PartialOrd, Enum)]
    pub enum ClubEditLevel {
        Editor = 0,
        Owner = 1,
    }
}

#[derive(Debug, Queryable, Identifiable, Insertable)]
#[table_name = "user_club_relation"]
#[primary_key(user_id, club_id)]
pub struct UserClubRelation {
    pub user_id: StringNumber,
    pub club_id: String,
    pub level: ClubEditLevel,
}
