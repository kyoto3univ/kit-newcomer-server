use async_graphql::{guard::Guard, Context, Object};
use diesel::{prelude::*, r2d2::ConnectionManager};
use r2d2::Pool;

use crate::{
    dto::paging::PagingObject,
    guard::PermissionGuard,
    models::{Asset, UserPermission},
};

#[derive(Debug, Default)]
pub struct AssetQuery;

#[Object]
impl AssetQuery {
    #[graphql(guard(PermissionGuard(permission = "UserPermission::ClubMember")))]
    async fn get_assets_from_club<'a>(
        &self,
        ctx: &'a Context<'_>,
        club_id: String,
        offset: Option<i64>,
        limit: Option<i64>,
    ) -> async_graphql::Result<PagingObject<Asset>> {
        let pool = ctx.data::<Pool<ConnectionManager<MysqlConnection>>>()?;
        let conn = pool.get()?;

        let assets = {
            use crate::models::schema::asset;

            let query = asset::table.filter(asset::club_id.eq(&club_id));

            (
                query.count().get_result::<i64>(&conn)?,
                query
                    .offset(offset.unwrap_or(0))
                    .limit(limit.unwrap_or(10))
                    .load::<Asset>(&conn)?,
            )
        };

        Ok(PagingObject {
            count: assets.0,
            items: assets.1,
        })
    }
}
