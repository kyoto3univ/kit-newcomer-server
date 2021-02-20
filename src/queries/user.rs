use async_graphql::{Context, Object, Result};
use diesel::{prelude::*, r2d2::ConnectionManager};
use r2d2::Pool;

use crate::models::{User, UserPermission};

#[derive(Debug, Default)]
pub struct UserQuery;

#[Object]
impl UserQuery {
    async fn me<'a>(&self, ctx: &'a Context<'_>) -> Result<User> {
        if let Ok(user) = ctx.data::<User>() {
            Ok(User {
                id: user.id,
                name: user.name.clone(),
                screen_name: user.screen_name.clone(),
                icon: user.icon.clone(),
                permission: user.permission.clone(),
                access_token: None,
                access_token_secret: None,
            })
        } else {
            Err(async_graphql::Error::new("Not logged in"))
        }
    }

    async fn get_users<'a>(
        &self,
        ctx: &'a Context<'_>,
        offset: Option<i64>,
        limit: Option<i64>,
        least_permission: Option<UserPermission>,
    ) -> Result<Vec<User>> {
        let pool = ctx.data::<Pool<ConnectionManager<MysqlConnection>>>()?;
        let conn = pool.get()?;

        let users = {
            use crate::models::schema::user;

            let mut filter = user::table
                .order(user::id.desc())
                .offset(offset.unwrap_or(0))
                .limit(limit.unwrap_or(10))
                .into_boxed();
            if let Some(perm) = least_permission {
                filter = filter.filter(user::permission.ge(perm));
            }

            filter.get_results::<User>(&conn)?
        };

        Ok(users)
    }
}
