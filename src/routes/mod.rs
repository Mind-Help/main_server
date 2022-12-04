use std::{collections::HashMap, io, sync::Arc};

// use async_graphql_axum::GraphQLSubscription;
use axum::{
	http::{HeaderValue, Method, StatusCode},
	response::IntoResponse,
	routing::{get, get_service, post},
	Router,
};
use tokio::sync::Mutex;
use tower_http::{cors::CorsLayer, services::ServeDir};

use crate::gql::schema::build_schema;

use handlers::graphql_handler;
#[cfg(debug_assertions)]
use handlers::graphql_playground;

use ws::{ws_handler, AppState};

mod handlers;
mod ws;

pub async fn build_routes() -> Router {
	let schema = build_schema().await;

	let app_state = Arc::new(AppState {
		slaves_connected: Mutex::new(HashMap::new()),
		users_connected: Mutex::new(HashMap::new()),
		ice_candidates: Mutex::new(Vec::new()),
	});

	let serve_dir = get_service(ServeDir::new("static")).handle_error(handle_error);
	let router = Router::new()
		.route("/", post(graphql_handler).with_state(schema))
		// .route("/ws", GraphQLSubscription::new(schema.clone()))
		.route("/ws", get(ws_handler).with_state(app_state))
		.nest_service("/test", serve_dir.clone())
		.fallback_service(serve_dir)
		.layer(
			CorsLayer::new()
				.allow_origin("*".parse::<HeaderValue>().unwrap())
				.allow_methods([
					Method::POST,
					#[cfg(debug_assertions)]
					Method::GET,
				]),
		);

	#[cfg(debug_assertions)]
	let router = router.route("/", get(graphql_playground));

	router
}

async fn handle_error(_err: io::Error) -> impl IntoResponse {
	(StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong...")
}
