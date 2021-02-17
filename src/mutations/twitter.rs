use async_graphql::{Context, Object, Result};
use egg_mode::{
    auth::{authorize_url, request_token},
    KeyPair,
};
use tokio_compat_02::FutureExt;

use crate::config::Config;
use crate::dto::twitter::TwitterAuthenticationResponse;

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
}
