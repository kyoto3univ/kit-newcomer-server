use async_graphql::{
    http::{playground_source, GraphQLPlaygroundConfig},
    EmptySubscription, Request, Schema,
};
use async_graphql_warp::{graphql, Response as GqlResponse};
use diesel::{r2d2::ConnectionManager, MysqlConnection};
use r2d2::Pool;
use std::{convert::Infallible, env};
use warp::{http::Response, Filter};

use super::mutations::*;
use super::queries::*;

type SchemaType = Schema<QueryRoot, MutationRoot, EmptySubscription>;
pub async fn start_graphql(db: Pool<ConnectionManager<MysqlConnection>>) {
    let schema = Schema::build(
        QueryRoot::default(),
        MutationRoot::default(),
        EmptySubscription,
    )
    .data(db)
    .finish();

    let gql_post =
        graphql(schema).and_then(|(schema, request): (SchemaType, Request)| async move {
            // Execute query
            let resp = schema.execute(request).await;

            // Return result
            Ok::<_, Infallible>(GqlResponse::from(resp))
        });
    let graphql_playground = warp::path::end().and(warp::get()).map(|| {
        Response::builder()
            .header("content-type", "text/html")
            .body(playground_source(GraphQLPlaygroundConfig::new("/")))
    });

    let filter = graphql_playground.or(gql_post);
    let port: u16 = env::var("PORT").map_or(8000, |s| s.parse().unwrap());

    warp::serve(filter).run(([0, 0, 0, 0], port)).await;
}
