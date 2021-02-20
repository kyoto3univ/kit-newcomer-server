use async_graphql::{Context, Object, Result};
use diesel::{prelude::*, r2d2::ConnectionManager};
use r2d2::Pool;
use uuid::Uuid;

use crate::{
    dto::club::NewClubDto,
    model_resolver::club::ClubWithMembers,
    models::{Club, ClubEditLevel, User, UserClubRelation},
};

#[derive(Debug, Default)]
pub struct ClubMutation;

#[Object]
impl ClubMutation {
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
}
