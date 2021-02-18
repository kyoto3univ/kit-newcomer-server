use async_graphql::{guard::Guard, Context, Error};
use diesel::{prelude::*, r2d2::ConnectionManager};
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use r2d2::Pool;
use warp::hyper::header::AUTHORIZATION;

use crate::{
    config::Config, dto::token::TokenClaim, models::User, models::UserPermission, utils::query,
};

pub struct PermissionGuard {
    permission: UserPermission,
}

#[async_trait::async_trait]
impl Guard for PermissionGuard {
    async fn check(&self, ctx: &Context<'_>) -> async_graphql::Result<()> {
        let config = ctx.data::<Config>()?;
        let pool = ctx.data::<Pool<ConnectionManager<MysqlConnection>>>()?;

        let headers = ctx.query_env.http_headers.lock();
        let token = headers
            .get(AUTHORIZATION)
            .ok_or_else(|| Error::new("Authorization header was not provided"))?
            .strip_prefix("Bearer ")
            .ok_or_else(|| Error::new("Authorization header is not valid"))?;

        let token_data = decode::<TokenClaim>(
            token,
            &DecodingKey::from_secret(config.jwt_secret.as_ref()),
            &Validation {
                leeway: 0,
                validate_exp: true,
                validate_nbf: false,
                algorithms: vec![Algorithm::HS256],
                iss: Some(config.jwt_issuer.clone()),
                aud: None,
                sub: None,
            },
        )?;

        let user = query(pool, move |conn| -> Result<User, anyhow::Error> {
            use crate::models::schema::user::dsl;
            Ok(dsl::user
                .find(token_data.claims.sub.parse::<i64>()?)
                .first::<User>(conn)?)
        })
        .await?;

        if user.permission >= self.permission {
            Ok(())
        } else {
            Err(Error::new("Not authorized"))
        }
    }
}
