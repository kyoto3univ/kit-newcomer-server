use async_graphql::{guard::Guard, Context, Error};

use crate::models::{User, UserPermission};

pub struct PermissionGuard {
    permission: UserPermission,
}

#[async_trait::async_trait]
impl Guard for PermissionGuard {
    async fn check(&self, ctx: &Context<'_>) -> async_graphql::Result<()> {
        let user = ctx.data::<User>()?;

        if user.permission >= self.permission {
            Ok(())
        } else {
            Err(Error::new("Insufficient permission"))
        }
    }
}
