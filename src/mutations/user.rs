use async_graphql::{guard::Guard, Context, Object};
use diesel::{prelude::*, r2d2::ConnectionManager};
use r2d2::Pool;

use crate::{
    guard::PermissionGuard,
    models::{User, UserPermission},
    utils::StringNumber,
};

#[derive(Debug, Default)]
pub struct UserMutation;

#[Object]
impl UserMutation {
    #[graphql(guard(PermissionGuard(permission = "UserPermission::Moderator")))]
    async fn update_user_permission<'a>(
        &self,
        ctx: &'a Context<'_>,
        user_id: StringNumber,
        permission: UserPermission,
    ) -> async_graphql::Result<User> {
        let pool = ctx.data::<Pool<ConnectionManager<MysqlConnection>>>()?;
        let user = ctx.data::<User>()?;
        let conn = pool.get()?;

        if permission > user.permission {
            return Err(async_graphql::Error::new("Not allowed"));
        }

        conn.transaction(|| -> Result<(), anyhow::Error> {
            use crate::models::schema::user::dsl;
            diesel::update(dsl::user.filter(dsl::id.eq(&user_id)))
                .set(dsl::permission.eq(permission))
                .execute(&conn)?;

            Ok(())
        })?;

        let user = {
            use crate::models::schema::user::dsl;
            dsl::user.find(&user_id).first::<User>(&conn)?
        };

        Ok(user)
    }
}
