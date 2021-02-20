use async_graphql::MergedObject;

mod club;
mod user;

#[derive(MergedObject, Default)]
pub struct QueryRoot(user::UserQuery, club::ClubQuery);
