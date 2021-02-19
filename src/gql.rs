use crate::{config::Config, dto::token::TokenClaim, models::User};
use async_graphql::{
    http::{playground_source, GraphQLPlaygroundConfig},
    EmptySubscription, Request, Response as GqlResponse, Schema, ServerError,
};
use async_graphql_warp::{graphql, Response as GqlWarpResponse};
use diesel::{prelude::*, r2d2::ConnectionManager};
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use r2d2::Pool;
use std::{env, sync::Arc};
use warp::{http::Response, Filter};

use super::mutations::*;
use super::queries::*;

type SchemaType = Schema<QueryRoot, MutationRoot, EmptySubscription>;
async fn get_user_from_token(
    auth_header: String,
    config: Arc<Config>,
    pool: Pool<ConnectionManager<MysqlConnection>>,
    request: Request,
) -> Result<Request, anyhow::Error> {
    let token = auth_header
        .strip_prefix("Bearer ")
        .ok_or_else(|| anyhow::anyhow!("Invalid header"))?;
    let conn = pool.get()?;

    let token_data = decode::<TokenClaim>(
        token,
        &DecodingKey::from_secret(config.jwt_secret.as_ref()),
        &Validation {
            leeway: 0,
            validate_exp: true,
            validate_nbf: false,
            algorithms: vec![Algorithm::HS256],
            iss: Some(config.jwt_issuer.clone()),
            aud: None,
            sub: None,
        },
    )?;

    let user: User = {
        use crate::models::schema::user::dsl;
        dsl::user
            .find(token_data.claims.sub.parse::<i64>()?)
            .first::<User>(&conn)?
    };

    Ok(request.data(user))
}

pub async fn start_graphql(cfg: Arc<Config>, db: Pool<ConnectionManager<MysqlConnection>>) {
    let schema = Schema::build(
        QueryRoot::default(),
        MutationRoot::default(),
        EmptySubscription,
    )
    .data(cfg.clone())
    .data(db.clone())
    .finish();

    let gql_post = warp::header::optional::<String>("authorization")
        .and(graphql(schema))
        .and_then(
            move |auth_header_opt: Option<String>, (schema, mut request): (SchemaType, Request)| {
                let cfg_clone = cfg.clone();
                let db_clone = db.clone();
                async move {
                    if let Some(auth_header) = auth_header_opt {
                        let result =
                            get_user_from_token(auth_header, cfg_clone, db_clone, request).await;
                        if let Ok(res) = result {
                            request = res;
                        } else {
                            return Ok(GqlWarpResponse::from(GqlResponse::from_errors(vec![
                                ServerError::new("Authorization header is not valid"),
                            ])));
                        }
                    }
                    let resp = schema.execute(request).await;

                    Ok::<_, warp::Rejection>(GqlWarpResponse::from(resp))
                }
            },
        );
    let graphql_playground = warp::path::end().and(warp::get()).map(|| {
        Response::builder()
            .header("content-type", "text/html")
            .body(playground_source(GraphQLPlaygroundConfig::new("/")))
    });

    let filter = graphql_playground.or(gql_post);
    let port: u16 = env::var("PORT").map_or(8000, |s| s.parse().unwrap());

    warp::serve(filter).run(([0, 0, 0, 0], port)).await;
}
