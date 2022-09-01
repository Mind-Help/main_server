use axum::{routing::get, Extension, Router};
use tower_http::trace::{DefaultMakeSpan, TraceLayer};

use crate::gql::schema::build_schema;

use handlers::{graphql_handler, graphql_playground, ws_handler};

mod handlers;

pub async fn build_routes() -> Router {
	let schema = build_schema().await;

	Router::new()
		.route("/", get(graphql_playground).post(graphql_handler))
		.layer(Extension(schema))
		.route("/chat", get(ws_handler))
		.layer(
			TraceLayer::new_for_http()
				.make_span_with(DefaultMakeSpan::default().include_headers(true)),
		)
}
