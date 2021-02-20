use async_graphql::{guard::Guard, Context, Object, Result};
use chrono::Utc;
use diesel::{dsl::count, prelude::*, r2d2::ConnectionManager};
use r2d2::Pool;
use uuid::Uuid;

use crate::{
    dto::club::{NewClubDto, UpdateClubDto},
    guard::PermissionGuard,
    model_resolver::club::ClubWithMembers,
    models::{Club, ClubEditLevel, User, UserClubRelation, UserPermission},
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

        let club_count = {
            use crate::models::schema::user_club_relation::*;
            table
                .filter(club_id.eq(&id).and(user_id.eq(user.id)))
                .select(count(club_id))
                .get_result::<i64>(&conn)?
        };
        if club_count == 0 && user.permission <= UserPermission::ClubMember {
            return Err(async_graphql::Error::new("Not allowed"));
        }

        conn.transaction(|| -> Result<(), anyhow::Error> {
            use crate::models::schema::club::dsl;
            diesel::update(dsl::club.filter(dsl::id.eq(&id)))
                .set((&update, dsl::updated_at.eq(Utc::now().naive_utc())))
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

        let club_count = {
            use crate::models::schema::user_club_relation::*;
            table
                .filter(
                    club_id
                        .eq(&id)
                        .and(user_id.eq(user.id))
                        .and(level.eq(ClubEditLevel::Owner)),
                )
                .select(count(club_id))
                .get_result::<i64>(&conn)?
        };
        if club_count == 0 && user.permission <= UserPermission::ClubMember {
            return Err(async_graphql::Error::new("Not allowed"));
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
}
