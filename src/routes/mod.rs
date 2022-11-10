use axum::{
	http::{HeaderValue, Method},
	routing::get,
	Extension, Router,
};
use tower_http::cors::CorsLayer;

use crate::gql::schema::build_schema;

use handlers::{graphql_handler, graphql_playground};

mod handlers;

pub async fn build_routes() -> Router {
	let schema = build_schema().await;

	Router::new()
		.route("/", get(graphql_playground).post(graphql_handler))
		.layer(
			CorsLayer::new()
				.allow_origin("*".parse::<HeaderValue>().unwrap())
				.allow_methods([
					Method::POST,
					#[cfg(debug_assertions)]
					Method::GET,
				]),
		)
		.layer(Extension(schema))
}
