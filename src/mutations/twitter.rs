use async_graphql::{Context, Object, Result};
use diesel::{prelude::*, r2d2::ConnectionManager, MysqlConnection, RunQueryDsl};
use egg_mode::{
    auth::{access_token, authorize_url, request_token},
    KeyPair,
};
use r2d2::Pool;
use tokio_compat_02::FutureExt;

use crate::utils::query;
use crate::{config::Config, dto::twitter::TwitterLoginInput};
use crate::{
    dto::twitter::{TwitterAuthenticationResponse, TwitterLoginResponse},
    models::{User, UserPermission},
};

#[derive(Default)]
pub struct TwitterAuthenticationMutation;

#[Object]
impl TwitterAuthenticationMutation {
    async fn get_authorize_info<'a>(
        &self,
        ctx: &'a Context<'_>,
    ) -> Result<TwitterAuthenticationResponse> {
        let config = ctx.data::<Config>()?;
        let keypair = KeyPair::new(
            config.twitter_consumer_key.clone(),
            config.twitter_consumer_secret.clone(),
        );
        let token = request_token(&keypair, config.twitter_callback.clone())
            .compat()
            .await?;

        Ok(TwitterAuthenticationResponse {
            request_token: String::from(token.key.clone()),
            request_token_secret: String::from(token.secret.clone()),
            callback_url: authorize_url(&token),
        })
    }

    async fn get_bearer_from_request_token<'a>(
        &self,
        ctx: &'a Context<'_>,
        request: TwitterLoginInput,
    ) -> Result<TwitterLoginResponse> {
        let config = ctx.data::<Config>()?;
        let pool = ctx.data::<Pool<ConnectionManager<MysqlConnection>>>()?;

        let ck_pair = KeyPair::new(
            config.twitter_consumer_key.clone(),
            config.twitter_consumer_secret.clone(),
        );
        let r_pair = KeyPair::new(request.request_token, request.request_token_secret);
        let (token, user_id, _) = access_token(ck_pair, &r_pair, request.verifier)
            .compat()
            .await?;

        if let egg_mode::auth::Token::Access {
            access,
            consumer: _,
        } = token.clone()
        {
            let tw_user = egg_mode::user::show(user_id, &token).compat().await?;

            query(pool, move |conn| -> Result<(), anyhow::Error> {
                use crate::models::schema::user::dsl;

                let find_result = dsl::user
                    .find(user_id as i64)
                    .first::<User>(conn)
                    .optional()?;

                if let Some(user) = find_result {
                    // Already signed up
                    diesel::update(&user)
                        .set((
                            dsl::icon.eq(Some(tw_user.response.profile_image_url_https)),
                            dsl::access_token.eq(Some(String::from(access.key.clone()))),
                            dsl::access_token_secret.eq(Some(String::from(access.secret.clone()))),
                            dsl::screen_name.eq(tw_user.response.screen_name),
                            dsl::name.eq(tw_user.response.name),
                        ))
                        .execute(conn)?;
                } else {
                    diesel::insert_into(dsl::user)
                        .values(User {
                            id: user_id as i64,
                            name: tw_user.response.name,
                            icon: Some(tw_user.response.profile_image_url_https),
                            screen_name: tw_user.response.screen_name,
                            permission: UserPermission::NewcomerOrNone,
                            access_token: Some(String::from(access.key.clone())),
                            access_token_secret: Some(String::from(access.secret.clone())),
                        })
                        .execute(conn)?;
                }

                Ok(())
            })
            .await?;

            Ok(TwitterLoginResponse {
                user_id: user_id.to_string(),
                bearer_token: String::from(""),
            })
        } else {
            Err(async_graphql::Error::new("Failed to get access token"))
        }
    }
}
