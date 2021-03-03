use async_graphql::{Context, Object, Result};
use chrono::{DateTime, Local, TimeZone};
use diesel::{prelude::*, r2d2::ConnectionManager};
use r2d2::Pool;

use crate::models::{
    Asset, Club, ClubEditLevel, ClubModerationState, ClubTopImageType, User, UserClubRelation,
    UserPermission,
};

pub struct ClubWithMembers(pub Club);

#[Object]
impl ClubWithMembers {
    // Tooooo long
    async fn id(&self) -> Result<&String> {
        Ok(&self.0.id)
    }
    async fn name(&self) -> Result<&String> {
        Ok(&self.0.name)
    }
    async fn is_published(&self) -> Result<&bool> {
        Ok(&self.0.is_published)
    }
    async fn moderation_state(&self) -> Result<&ClubModerationState> {
        Ok(&self.0.moderation_state)
    }
    async fn short_description(&self) -> Result<&Option<String>> {
        Ok(&self.0.short_description)
    }
    async fn long_description(&self) -> Result<&Option<String>> {
        Ok(&self.0.long_description)
    }
    async fn join_description(&self) -> Result<&Option<String>> {
        Ok(&self.0.join_description)
    }
    async fn place(&self) -> Result<&Option<String>> {
        Ok(&self.0.place)
    }
    async fn schedule(&self) -> Result<&Option<String>> {
        Ok(&self.0.schedule)
    }
    async fn video_url(&self) -> Result<&Option<String>> {
        Ok(&self.0.video_url)
    }
    async fn contact_url(&self) -> Result<&Option<String>> {
        Ok(&self.0.contact_url)
    }
    async fn created_at(&self) -> Result<DateTime<Local>> {
        Ok(Local.from_utc_datetime(&self.0.created_at))
    }
    async fn updated_at(&self) -> Result<DateTime<Local>> {
        Ok(Local.from_utc_datetime(&self.0.updated_at))
    }
    async fn top_content_type(&self) -> Result<&ClubTopImageType> {
        Ok(&self.0.top_content_type)
    }

    // Meaningful impl
    async fn members<'b>(&self, ctx: &'b Context<'_>) -> Result<Vec<ClubWithMembersItem>> {
        let conn = ctx
            .data::<Pool<ConnectionManager<MysqlConnection>>>()?
            .get()?;

        let related_users = {
            use crate::models::schema::{user, user_club_relation};
            user_club_relation::table
                .inner_join(user::table)
                .filter(user_club_relation::club_id.eq(&self.0.id))
                .load::<(UserClubRelation, User)>(&conn)?
        };

        Ok(related_users
            .into_iter()
            .map(|(rel, user)| ClubWithMembersItem(rel.level, user))
            .collect())
    }

    async fn top_image<'b>(&self, ctx: &'b Context<'_>) -> Result<Option<Asset>> {
        if self.0.top_image_id.is_none() {
            return Ok(None);
        }

        let conn = ctx
            .data::<Pool<ConnectionManager<MysqlConnection>>>()?
            .get()?;

        let asset = {
            use crate::models::schema::asset;

            asset::table
                .find(self.0.top_image_id.unwrap())
                .first::<Asset>(&conn)?
        };

        Ok(Some(asset))
    }

    async fn thumb_image<'b>(&self, ctx: &'b Context<'_>) -> Result<Option<Asset>> {
        if self.0.thumb_image_id.is_none() {
            return Ok(None);
        }

        let conn = ctx
            .data::<Pool<ConnectionManager<MysqlConnection>>>()?
            .get()?;

        let asset = {
            use crate::models::schema::asset;

            asset::table
                .find(self.0.thumb_image_id.unwrap())
                .first::<Asset>(&conn)?
        };

        Ok(Some(asset))
    }
}

pub struct ClubWithMembersItem(pub ClubEditLevel, pub User);

#[Object]
impl ClubWithMembersItem {
    async fn level(&self) -> Result<&ClubEditLevel> {
        if self.1.permission >= UserPermission::Moderator {
            Ok(&ClubEditLevel::Owner)
        } else {
            Ok(&self.0)
        }
    }

    async fn user(&self) -> Result<&User> {
        Ok(&self.1)
    }
}

pub struct ClubWithLevelItem(pub ClubEditLevel, pub Club);

#[Object]
impl ClubWithLevelItem {
    async fn level(&self) -> Result<&ClubEditLevel> {
        Ok(&self.0)
    }

    async fn club(&self) -> Result<ClubWithMembers> {
        Ok(ClubWithMembers(self.1.clone()))
    }
}
