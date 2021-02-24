use async_graphql::MergedObject;

mod asset;
mod club;
mod user;

#[derive(MergedObject, Default)]
pub struct QueryRoot(user::UserQuery, club::ClubQuery, asset::AssetQuery);
