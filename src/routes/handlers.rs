use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{
	extract::{
		ws::{Message, WebSocket},
		WebSocketUpgrade,
	},
	headers::UserAgent,
	response::{Html, IntoResponse},
	Extension, TypedHeader,
};

use crate::gql::schema::AppSchema;

pub async fn graphql_handler(schema: Extension<AppSchema>, req: GraphQLRequest) -> GraphQLResponse {
	schema.execute(req.into_inner()).await.into()
}

pub async fn graphql_playground() -> impl IntoResponse {
	Html(playground_source(GraphQLPlaygroundConfig::new("/")))
}

pub async fn ws_handler(
	ws: WebSocketUpgrade,
	user_agent: Option<TypedHeader<UserAgent>>,
) -> impl IntoResponse {
	if let Some(TypedHeader(user_agent)) = user_agent {
		println!("`{}` connected", user_agent.as_str());
	}

	ws.on_upgrade(handle_socket)
}

async fn handle_socket(mut socket: WebSocket) {
	if let Some(msg) = socket.recv().await {
		if let Ok(msg) = msg {
			match msg {
				Message::Text(t) => {
					println!("client sent str: {:?}", t);
				}
				Message::Binary(_) => {
					println!("client sent binary data");
				}
				Message::Ping(_) => {
					println!("socket ping");
				}
				Message::Pong(_) => {
					println!("socket pong");
				}
				Message::Close(_) => {
					println!("client disconnected");
					return;
				}
			}
		} else {
			println!("client disconnected");
			return;
		}
	}

	loop {
		if socket
			.send(Message::Text(String::from("Hi!")))
			.await
			.is_err()
		{
			println!("client disconnected");
			return;
		}
		tokio::time::sleep(std::time::Duration::from_secs(3)).await;
	}
}
