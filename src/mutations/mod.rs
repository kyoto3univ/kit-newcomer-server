use async_graphql::MergedObject;
use twitter::TwitterAuthenticationMutation;

mod club;
mod twitter;
mod user;
mod asset;

#[derive(MergedObject, Default)]
pub struct MutationRoot(
    TwitterAuthenticationMutation,
    club::ClubMutation,
    user::UserMutation,
);
