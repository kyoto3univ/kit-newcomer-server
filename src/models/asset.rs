use super::schema::asset;
use crate::utils::StringNumber;
use async_graphql::SimpleObject;
use diesel::{dsl::count, prelude::*, r2d2::ConnectionManager, Identifiable, Queryable};
use r2d2::PooledConnection;

#[derive(Debug, Queryable, Associations, Identifiable, Insertable, SimpleObject)]
#[belongs_to(super::Club, foreign_key = "club_id")]
#[belongs_to(super::User, foreign_key = "owner_id")]
#[table_name = "asset"]
pub struct Asset {
    pub id: StringNumber,
    pub owner_id: StringNumber,
    pub club_id: String,
    pub name: String,
    pub alternative_description: Option<String>,
    pub file_path: String,
    pub file_size: i32,
    pub image_width: Option<i32>,
    pub image_height: Option<i32>,
}

impl Asset {
    pub fn check_permission(
        conn: &PooledConnection<ConnectionManager<MysqlConnection>>,
        asset_id: &StringNumber,
        club_id: &String,
    ) -> Result<bool, anyhow::Error> {
        let asset_count = {
            asset::table
                .filter(asset::id.eq(asset_id.0).and(asset::club_id.eq(club_id)))
                .select(count(asset::club_id))
                .get_result::<i64>(conn)?
        };

        Ok(asset_count == 1)
    }
}
