use std::{
    io::{Read, Write},
    path::Path,
    sync::Arc,
};

use async_graphql::{guard::Guard, Context, Object, Upload};
use diesel::{prelude::*, r2d2::ConnectionManager};
use infer::MatcherType;
use r2d2::Pool;
use uuid::Uuid;

use crate::{
    config::Config,
    dto::asset::{NewAssetDto, UpdateAssetDto},
    guard::PermissionGuard,
    models::{last_insert_id, Asset, Club, User, UserPermission},
    utils::StringNumber,
};

#[derive(Debug, Default)]
pub struct AssetMutation;

#[Object]
impl AssetMutation {
    #[graphql(guard(PermissionGuard(permission = "UserPermission::ClubMember")))]
    async fn upload_asset_for<'a>(
        &self,
        ctx: &'a Context<'_>,
        club_id: String,
        upload: Upload,
    ) -> async_graphql::Result<Asset> {
        let config = ctx.data::<Arc<Config>>()?;
        let pool = ctx.data::<Pool<ConnectionManager<MysqlConnection>>>()?;
        let user = ctx.data::<User>()?;
        let conn = pool.get()?;

        if !Club::check_club_permission(
            &conn,
            &club_id,
            user,
            crate::models::ClubEditLevel::Editor,
        )? {
            return Err(async_graphql::Error::new("Not allowed"));
        }

        let upload_value = upload.value(ctx)?;
        let upload_size = upload_value.size()?;
        let upload_filename = upload_value.filename.clone();
        if upload_size >= 1024 * 1024 * 5 {
            return Err(async_graphql::Error::new("File size exceeded"));
        }

        let mut file = upload_value.content;
        let mut buf: Vec<u8> = Vec::with_capacity(upload_size as usize);
        let read_size = file.read_to_end(&mut buf)?;

        if read_size != upload_size as usize {
            return Err(async_graphql::Error::new("File read failed"));
        }

        let ftype =
            infer::get(&buf).ok_or_else(|| async_graphql::Error::new("File type is unknown"))?;

        if ftype.matcher_type() != MatcherType::IMAGE {
            return Err(async_graphql::Error::new("File type is not allowed"));
        }

        let mut file_name = Uuid::new_v4().to_hyphenated().to_string();
        file_name.push('.');
        file_name.push_str(ftype.extension());

        let file_path = Path::new(&config.asset_path)
            .to_path_buf()
            .join(file_name.clone());
        let file_path_str = file_path
            .to_str()
            .ok_or_else(|| async_graphql::Error::new("Path construction error"))?;

        let mut file = std::fs::File::create(file_path_str)?;

        file.write_all(&buf)?;
        file.flush()?;

        let asset_id = conn.transaction(|| -> Result<i64, anyhow::Error> {
            use crate::models::schema::asset;
            diesel::insert_into(asset::table)
                .values(&NewAssetDto {
                    owner_id: user.id,
                    club_id: club_id.clone(),
                    name: upload_filename,
                    file_path: file_name,
                    file_size: upload_size as i32,
                })
                .execute(&conn)?;

            Ok(diesel::select(last_insert_id).first::<i64>(&conn)?)
        })?;

        let asset = {
            use crate::models::schema::asset;
            asset::table.find(asset_id).get_result::<Asset>(&conn)?
        };

        Ok(asset)
    }

    #[graphql(guard(PermissionGuard(permission = "UserPermission::ClubMember")))]
    async fn update_asset<'a>(
        &self,
        ctx: &'a Context<'_>,
        asset_id: StringNumber,
        update: UpdateAssetDto,
    ) -> async_graphql::Result<Asset> {
        let pool = ctx.data::<Pool<ConnectionManager<MysqlConnection>>>()?;
        let user = ctx.data::<User>()?;
        let conn = pool.get()?;

        let club_id = {
            use crate::models::schema::asset;

            asset::table
                .find(asset_id)
                .select(asset::club_id)
                .first::<String>(&conn)?
        };

        if !Club::check_club_permission(
            &conn,
            &club_id,
            user,
            crate::models::ClubEditLevel::Editor,
        )? {
            return Err(async_graphql::Error::new("Not allowed"));
        }

        conn.transaction(|| -> Result<(), anyhow::Error> {
            use crate::models::schema::asset;
            diesel::update(asset::table.find(asset_id))
                .set(&update)
                .execute(&conn)?;

            Ok(())
        })?;

        let asset = {
            use crate::models::schema::asset;
            asset::table.find(asset_id).get_result::<Asset>(&conn)?
        };

        Ok(asset)
    }
}
