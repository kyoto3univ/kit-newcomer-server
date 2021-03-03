use async_graphql::{guard::Guard, Context, Object, Result};
use diesel::{prelude::*, r2d2::ConnectionManager};
use r2d2::Pool;

use crate::dto::paging::PagingObject;
use crate::guard::PermissionGuard;
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

    #[graphql(guard(PermissionGuard(permission = "UserPermission::ClubMember")))]
    async fn get_users<'a>(
        &self,
        ctx: &'a Context<'_>,
        offset: Option<i64>,
        limit: Option<i64>,
        least_permission: Option<UserPermission>,
        screen_name: Option<String>,
    ) -> Result<PagingObject<User>> {
        let pool = ctx.data::<Pool<ConnectionManager<MysqlConnection>>>()?;
        let conn = pool.get()?;

        let users = {
            use crate::models::schema::user;

            let make_filter = || {
                let mut filter = user::table.order(user::id.desc()).into_boxed();
                if let Some(perm) = &least_permission {
                    filter = filter.filter(user::permission.ge(perm));
                }

                if let Some(sn) = &screen_name {
                    filter = filter.filter(user::screen_name.like(format!("%{}%", sn)));
                }
                filter
            };

            PagingObject {
                count: make_filter().count().get_result::<i64>(&conn)?,
                items: make_filter()
                    .offset(offset.unwrap_or(0))
                    .limit(limit.unwrap_or(10))
                    .get_results::<User>(&conn)?,
            }
        };

        Ok(users)
    }
}
