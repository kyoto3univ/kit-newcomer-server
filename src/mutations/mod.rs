use async_graphql::MergedObject;
use twitter::TwitterAuthenticationMutation;

mod twitter;

#[derive(MergedObject, Default)]
pub struct MutationRoot(TwitterAuthenticationMutation);
