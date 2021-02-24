use async_graphql::MergedObject;
use twitter::TwitterAuthenticationMutation;

mod asset;
mod club;
mod twitter;
mod user;

#[derive(MergedObject, Default)]
pub struct MutationRoot(
    TwitterAuthenticationMutation,
    club::ClubMutation,
    user::UserMutation,
    asset::AssetMutation,
);
