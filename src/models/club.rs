use crate::define_enum;

use super::{schema::club, ClubEditLevel, User, UserPermission};
use async_graphql::{Enum, Result};
use chrono::NaiveDateTime;
use diesel::{dsl::count, prelude::*, r2d2::ConnectionManager, Identifiable, Queryable};
use r2d2::PooledConnection;

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

impl Club {
    pub fn check_club_permission(
        conn: &PooledConnection<ConnectionManager<MysqlConnection>>,
        id: &String,
        user: &User,
        perm: ClubEditLevel,
    ) -> Result<bool> {
        if user.permission >= UserPermission::Moderator {
            return Ok(true);
        }

        let club_count = {
            use crate::models::schema::user_club_relation::*;
            table
                .filter(club_id.eq(id).and(user_id.eq(user.id)).and(level.ge(perm)))
                .select(count(club_id))
                .get_result::<i64>(conn)?
        };

        Ok(club_count == 1)
    }
}
