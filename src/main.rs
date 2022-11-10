use routes::build_routes;

mod db;
mod gql;
mod routes;
mod utils;

#[tokio::main]
async fn main() {
	let app = build_routes().await;
	let addr = format!("[::]:{}", env!("PORT"))
		.parse()
		.expect("INVALID PORT");

	axum::Server::bind(&addr)
		.serve(app.into_make_service())
		.await
		.unwrap();
}
