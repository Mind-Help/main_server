use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{
	response::{Html, IntoResponse},
	Extension,
};

use crate::gql::schema::AppSchema;

pub async fn graphql_handler(schema: Extension<AppSchema>, req: GraphQLRequest) -> GraphQLResponse {
	schema.execute(req.into_inner()).await.into()
}

pub async fn graphql_playground() -> impl IntoResponse {
	Html(playground_source(GraphQLPlaygroundConfig::new("/")))
}
