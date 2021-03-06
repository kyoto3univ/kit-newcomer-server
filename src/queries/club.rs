use async_graphql::{guard::Guard, Context, Object, Result};
use diesel::{prelude::*, r2d2::ConnectionManager};
use r2d2::Pool;

use crate::{
    dto::paging::PagingObject,
    guard::PermissionGuard,
    model_resolver::club::{ClubWithLevelItem, ClubWithMembers},
    models::{Club, ClubModerationState, User, UserClubRelation, UserPermission},
};

#[derive(Debug, Default)]
pub struct ClubQuery;

#[Object]
impl ClubQuery {
    #[graphql(guard(PermissionGuard(permission = "UserPermission::ClubMember")))]
    async fn get_my_clubs<'a>(&self, ctx: &'a Context<'_>) -> Result<Vec<ClubWithLevelItem>> {
        let pool = ctx.data::<Pool<ConnectionManager<MysqlConnection>>>()?;
        let user = ctx.data::<User>()?;
        let conn = pool.get()?;

        let clubs = {
            use crate::models::schema::{club, user_club_relation};

            user_club_relation::table
                .inner_join(club::table)
                .filter(user_club_relation::user_id.eq(user.id.0))
                .order(club::created_at.desc())
                .load::<(UserClubRelation, Club)>(&conn)?
        };

        Ok(clubs
            .into_iter()
            .map(|club| ClubWithLevelItem(club.0.level, club.1))
            .collect())
    }

    async fn get_clubs<'a>(
        &self,
        ctx: &'a Context<'_>,
        offset: Option<i64>,
        limit: Option<i64>,
        include_unpublished: Option<bool>,
        moderation_state: Option<ClubModerationState>,
    ) -> Result<PagingObject<ClubWithMembers>> {
        let pool = ctx.data::<Pool<ConnectionManager<MysqlConnection>>>()?;
        let conn = pool.get()?;

        if include_unpublished.unwrap_or(false) || moderation_state.is_some() {
            let user = ctx.data::<User>()?;
            if user.permission < UserPermission::Moderator {
                return Err(async_graphql::Error::new("Not allowed"));
            }
        }

        let clubs = {
            use crate::models::schema::club;

            let query = || {
                let mut query = club::table.order(club::created_at.desc()).into_boxed();
                if !include_unpublished.unwrap_or(false) {
                    query = query.filter(club::is_published.eq(true));
                }

                if let Some(state) = &moderation_state {
                    query = query.filter(club::moderation_state.eq(state));
                }

                query
            };

            (
                query().count().get_result::<i64>(&conn)?,
                query()
                    .offset(offset.unwrap_or(0))
                    .limit(limit.unwrap_or(10))
                    .load::<Club>(&conn)?,
            )
        };

        Ok(PagingObject {
            count: clubs.0,
            items: clubs
                .1
                .into_iter()
                .map(|club| ClubWithMembers(club))
                .collect(),
        })
    }

    async fn club<'a>(&self, ctx: &'a Context<'_>, id: String) -> Result<ClubWithMembers> {
        let pool = ctx.data::<Pool<ConnectionManager<MysqlConnection>>>()?;
        let conn = pool.get()?;

        let club = {
            use crate::models::schema::club;

            club::table.find(&id).get_result::<Club>(&conn)?
        };

        if !club.is_published {
            let user = ctx.data::<User>()?;
            if user.permission < UserPermission::Moderator {
                let is_member = {
                    use crate::models::schema::user_club_relation;

                    user_club_relation::table
                        .filter(
                            user_club_relation::club_id
                                .eq(&id)
                                .and(user_club_relation::user_id.eq(&user.id)),
                        )
                        .count()
                        .get_result::<i64>(&conn)?
                        > 0
                };

                if !is_member {
                    return Err(async_graphql::Error::new("Not allowed"));
                }
            }
        }

        Ok(ClubWithMembers(club))
    }
}
