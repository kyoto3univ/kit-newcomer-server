use async_graphql::SimpleObject;

#[derive(Debug, SimpleObject)]
pub struct TwitterAuthenticationResponse {
    pub request_token: String,
    pub request_token_secret: String,
    pub callback_url: String,
}
