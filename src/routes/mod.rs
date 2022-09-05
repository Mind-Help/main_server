use axum::{routing::get, Extension, Router};

use crate::gql::schema::build_schema;

use handlers::{graphql_handler, graphql_playground};

mod handlers;

pub async fn build_routes() -> Router {
	let schema = build_schema().await;

	Router::new()
		.route("/", get(graphql_playground).post(graphql_handler))
		.layer(Extension(schema))
}
