use routes::build_routes;
use std::env::var;

mod db;
mod gql;
mod routes;

#[tokio::main]
async fn main() {
	dotenv::dotenv().ok();

	let app = build_routes().await;
	let addr = format!(
		"[::]:{}",
		var("PORT").unwrap_or_else(|_| {
			eprintln!("$PORT not found, using 3000 as default.");
			String::from("3000")
		})
	)
	.parse()
	.expect("INVALID PORT");

	axum::Server::bind(&addr)
		.serve(app.into_make_service())
		.await
		.unwrap();
}
