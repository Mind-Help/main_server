#[cfg(debug_assertions)]
use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::extract::State;
#[cfg(debug_assertions)]
use axum::response::{Html, IntoResponse};

use crate::gql::schema::AppSchema;

pub async fn graphql_handler(schema: State<AppSchema>, req: GraphQLRequest) -> GraphQLResponse {
	schema.execute(req.into_inner()).await.into()
}

#[cfg(debug_assertions)]
pub async fn graphql_playground() -> impl IntoResponse {
	Html(playground_source(GraphQLPlaygroundConfig::new("/")))
}
