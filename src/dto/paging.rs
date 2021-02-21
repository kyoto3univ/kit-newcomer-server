use crate::{model_resolver::club::ClubWithMembers, models::User};
use async_graphql::{OutputType, SimpleObject};

#[derive(SimpleObject)]
#[graphql(concrete(name = "UserPaging", params(User)))]
#[graphql(concrete(name = "ClubPaging", params(ClubWithMembers)))]
pub struct PagingObject<T: OutputType> {
    pub items: Vec<T>,
    pub count: i64,
}
