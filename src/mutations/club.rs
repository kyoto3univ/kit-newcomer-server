use async_graphql::{guard::Guard, Context, Object, Result};
use chrono::Utc;
use diesel::{prelude::*, r2d2::ConnectionManager};
use r2d2::Pool;
use uuid::Uuid;

use crate::{
    dto::club::{NewClubDto, UpdateClubDbDto, UpdateClubDto},
    guard::PermissionGuard,
    model_resolver::club::ClubWithMembers,
    models::{Club, ClubEditLevel, ClubModerationState, User, UserClubRelation, UserPermission},
    utils::StringNumber,
};

#[derive(Debug, Default)]
pub struct ClubMutation;

#[Object]
impl ClubMutation {
    #[graphql(guard(PermissionGuard(permission = "UserPermission::ClubMember")))]
    async fn create_new_club<'a>(
        &self,
        ctx: &'a Context<'_>,
        name: String,
    ) -> Result<ClubWithMembers> {
        let pool = ctx.data::<Pool<ConnectionManager<MysqlConnection>>>()?;
        let user = ctx.data::<User>()?;
        let conn = pool.get()?;

        let id = conn.transaction(|| -> Result<String, anyhow::Error> {
            let id = Uuid::new_v4().to_hyphenated().to_string();
            {
                use crate::models::schema::club::dsl;
                diesel::insert_into(dsl::club)
                    .values(&NewClubDto {
                        id: &id,
                        name: &name,
                        is_published: false,
                    })
                    .execute(&conn)?;
            }
            {
                use crate::models::schema::user_club_relation::dsl;
                diesel::insert_into(dsl::user_club_relation)
                    .values(UserClubRelation {
                        user_id: user.id.clone(),
                        club_id: id.clone(),
                        level: ClubEditLevel::Owner,
                    })
                    .execute(&conn)?;
            }
            Ok(id)
        })?;

        let club: Club = {
            use crate::models::schema::club::dsl;
            dsl::club.find(id).first::<Club>(&conn)?
        };

        Ok(ClubWithMembers(club))
    }

    #[graphql(guard(PermissionGuard(permission = "UserPermission::ClubMember")))]
    async fn update_club<'a>(
        &self,
        ctx: &'a Context<'_>,
        id: String,
        update: UpdateClubDto,
    ) -> Result<ClubWithMembers> {
        let pool = ctx.data::<Pool<ConnectionManager<MysqlConnection>>>()?;
        let user = ctx.data::<User>()?;
        let conn = pool.get()?;

        if !Club::check_club_permission(&conn, &id, user, ClubEditLevel::Editor)? {
            return Err(async_graphql::Error::new("Not allowed"));
        }

        if !update.validate(&conn, &id)? {
            return Err(async_graphql::Error::new("Not allowed"));
        }

        conn.transaction(|| -> Result<(), anyhow::Error> {
            use crate::models::schema::club::dsl;
            diesel::update(dsl::club.filter(dsl::id.eq(&id)))
                .set((
                    UpdateClubDbDto::from(update),
                    dsl::updated_at.eq(Utc::now().naive_utc()),
                ))
                .execute(&conn)?;

            Ok(())
        })?;

        let club = {
            use crate::models::schema::club::dsl;
            dsl::club.find(&id).get_result::<Club>(&conn)?
        };

        Ok(ClubWithMembers(club))
    }

    #[graphql(guard(PermissionGuard(permission = "UserPermission::ClubMember")))]
    async fn change_publish_state<'a>(
        &self,
        ctx: &'a Context<'_>,
        id: String,
        is_published: bool,
    ) -> Result<ClubWithMembers> {
        let pool = ctx.data::<Pool<ConnectionManager<MysqlConnection>>>()?;
        let user = ctx.data::<User>()?;
        let conn = pool.get()?;

        if !Club::check_club_permission(&conn, &id, user, ClubEditLevel::Owner)? {
            return Err(async_graphql::Error::new("Not allowed"));
        }

        let club: Club = {
            use crate::models::schema::club::dsl;
            dsl::club.find(&id).first::<Club>(&conn)?
        };

        if club.moderation_state != ClubModerationState::Accepted {
            return Err(async_graphql::Error::new("Not accepted"));
        }

        conn.transaction(|| -> Result<(), anyhow::Error> {
            use crate::models::schema::club::dsl;
            diesel::update(dsl::club.filter(dsl::id.eq(&id)))
                .set(dsl::is_published.eq(&is_published))
                .execute(&conn)?;

            Ok(())
        })?;

        let club = {
            use crate::models::schema::club::dsl;
            dsl::club.find(&id).get_result::<Club>(&conn)?
        };

        Ok(ClubWithMembers(club))
    }

    #[graphql(guard(PermissionGuard(permission = "UserPermission::ClubMember")))]
    async fn change_moderation_state<'a>(
        &self,
        ctx: &'a Context<'_>,
        id: String,
        moderation_state: ClubModerationState,
    ) -> Result<ClubWithMembers> {
        let pool = ctx.data::<Pool<ConnectionManager<MysqlConnection>>>()?;
        let user = ctx.data::<User>()?;
        let conn = pool.get()?;

        if !Club::check_club_permission(&conn, &id, user, ClubEditLevel::Owner)? {
            return Err(async_graphql::Error::new("Not allowed"));
        }

        // Less than moderater & more than waiting state
        if moderation_state > ClubModerationState::Waiting
            && user.permission < UserPermission::Moderator
        {
            return Err(async_graphql::Error::new("Not accepted"));
        }

        conn.transaction(|| -> Result<(), anyhow::Error> {
            use crate::models::schema::club::dsl;
            diesel::update(dsl::club.filter(dsl::id.eq(&id)))
                .set(dsl::moderation_state.eq(&moderation_state))
                .execute(&conn)?;

            Ok(())
        })?;

        let club = {
            use crate::models::schema::club::dsl;
            dsl::club.find(&id).get_result::<Club>(&conn)?
        };

        Ok(ClubWithMembers(club))
    }

    #[graphql(guard(PermissionGuard(permission = "UserPermission::ClubMember")))]
    async fn add_member_to_club<'a>(
        &self,
        ctx: &'a Context<'_>,
        club_id: String,
        user_id: StringNumber,
        level: Option<ClubEditLevel>,
    ) -> Result<ClubWithMembers> {
        let pool = ctx.data::<Pool<ConnectionManager<MysqlConnection>>>()?;
        let user = ctx.data::<User>()?;
        let conn = pool.get()?;

        if !Club::check_club_permission(&conn, &club_id, user, ClubEditLevel::Owner)? {
            return Err(async_graphql::Error::new("Not allowed"));
        }

        conn.transaction(|| -> Result<()> {
            use crate::models::schema::{user, user_club_relation};

            // Update user automatically when it is not a club member role.
            diesel::update(
                user::table.filter(
                    user::id
                        .eq(&user_id)
                        .and(user::permission.lt(UserPermission::ClubMember)),
                ),
            )
            .set(user::permission.eq(UserPermission::ClubMember))
            .execute(&conn)?;

            diesel::insert_or_ignore_into(user_club_relation::table)
                .values(&UserClubRelation {
                    club_id: club_id.clone(),
                    user_id: user_id,
                    level: level.unwrap_or(ClubEditLevel::Editor),
                })
                .execute(&conn)?;
            Ok(())
        })?;

        let club = {
            use crate::models::schema::club::dsl;
            dsl::club.find(&club_id).get_result::<Club>(&conn)?
        };

        Ok(ClubWithMembers(club))
    }

    #[graphql(guard(PermissionGuard(permission = "UserPermission::ClubMember")))]
    async fn delete_member_from_club<'a>(
        &self,
        ctx: &'a Context<'_>,
        club_id: String,
        user_id: StringNumber,
    ) -> Result<ClubWithMembers> {
        let pool = ctx.data::<Pool<ConnectionManager<MysqlConnection>>>()?;
        let user = ctx.data::<User>()?;
        let conn = pool.get()?;

        if !Club::check_club_permission(&conn, &club_id, user, ClubEditLevel::Owner)? {
            return Err(async_graphql::Error::new("Not allowed"));
        }

        if user_id.0 == user.id.0 {
            return Err(async_graphql::Error::new("User must not be you"));
        }

        conn.transaction(|| -> std::result::Result<(), anyhow::Error> {
            use crate::models::schema::user_club_relation::{dsl, table};

            let result = diesel::delete(
                table.filter(dsl::user_id.eq(&user_id).and(dsl::club_id.eq(&club_id))),
            )
            .execute(&conn)?;

            if result > 0 {
                Ok(())
            } else {
                Err(anyhow::anyhow!("No such relation"))
            }
        })?;

        let club = {
            use crate::models::schema::club::dsl;
            dsl::club.find(&club_id).get_result::<Club>(&conn)?
        };

        Ok(ClubWithMembers(club))
    }
}
