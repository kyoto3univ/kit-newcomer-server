use async_graphql::{Context, Object, Result};

use crate::models::User;

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
}
