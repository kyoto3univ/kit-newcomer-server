use async_graphql::{Context, Object, Result};
use diesel::{prelude::*, r2d2::ConnectionManager};
use r2d2::Pool;

use crate::{
    model_resolver::club::ClubWithLevelItem,
    models::{Club, User, UserClubRelation},
};

#[derive(Debug, Default)]
pub struct ClubQuery;

#[Object]
impl ClubQuery {
    async fn get_my_clubs<'a>(&self, ctx: &'a Context<'_>) -> Result<Vec<ClubWithLevelItem>> {
        let pool = ctx.data::<Pool<ConnectionManager<MysqlConnection>>>()?;
        let user = ctx.data::<User>()?;
        let conn = pool.get()?;

        let clubs = {
            use crate::models::schema::{club, user_club_relation};

            user_club_relation::table
                .inner_join(club::table)
                .filter(user_club_relation::user_id.eq(user.id.0))
                .load::<(UserClubRelation, Club)>(&conn)?
        };

        Ok(clubs
            .into_iter()
            .map(|club| ClubWithLevelItem(club.0.level, club.1))
            .collect())
    }
}
