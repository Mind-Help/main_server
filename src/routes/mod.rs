#[cfg(debug_assertions)]
use axum::routing::get;
use axum::{
	http::{HeaderValue, Method},
	routing::post,
	Extension, Router,
};
use tower_http::cors::CorsLayer;

use crate::gql::schema::build_schema;

use handlers::graphql_handler;
#[cfg(debug_assertions)]
use handlers::graphql_playground;

mod handlers;

pub async fn build_routes() -> Router {
	let schema = build_schema().await;

	let router = Router::new()
		.route("/", post(graphql_handler))
		.layer(
			CorsLayer::new()
				.allow_origin("*".parse::<HeaderValue>().unwrap())
				.allow_methods([
					Method::POST,
					#[cfg(debug_assertions)]
					Method::GET,
				]),
		)
		.layer(Extension(schema));

	#[cfg(debug_assertions)]
	let router = router.route("/", get(graphql_playground));

	router
}
