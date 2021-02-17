use async_graphql::{InputObject, SimpleObject};

#[derive(Debug, SimpleObject)]
pub struct TwitterAuthenticationResponse {
    pub request_token: String,
    pub request_token_secret: String,
    pub callback_url: String,
}

#[derive(Debug, InputObject)]
pub struct TwitterLoginInput {
    pub request_token: String,
    pub request_token_secret: String,
    pub verifier: String,
}

#[derive(Debug, SimpleObject)]
pub struct TwitterLoginResponse {
    pub bearer_token: String,
    pub user_id: String,
}
