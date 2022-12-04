use std::{str::FromStr, sync::Arc};

use axum::{
	extract::{
		ws::{Message, WebSocket},
		State, WebSocketUpgrade,
	},
	response::IntoResponse,
};
#[cfg(debug_assertions)]
use axum::{headers::UserAgent, TypedHeader};
use chrono::Utc;
use serde_json::{from_str, to_string};
use tokio::sync::Mutex;
use uuid::Uuid;

pub use datatypes::AppState;
use datatypes::{
	ConData, IceReq, Queue, QueueMatch, Req, ReqMessageType, Res, ResMessageType, UserConnected,
	WebRTCAnswer, WebRTCConReq, WebRTCPCEstablished,
};

mod datatypes;

pub async fn ws_handler(
	ws: WebSocketUpgrade,
	#[cfg(debug_assertions)] user_agent: Option<TypedHeader<UserAgent>>,
	State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
	#[cfg(debug_assertions)]
	if let Some(TypedHeader(user_agent)) = user_agent {
		println!("`{}` connected", user_agent.as_str());
	}

	ws.on_upgrade(|socket| handle_socket(socket, state))
}

async fn handle_socket(socket: WebSocket, state: Arc<AppState>) {
	let get_target = |queue: &Queue, target_id: &str| {
		queue
			.keys()
			.find(|u| u.id.to_string().eq(target_id))
			.map(|data| data.to_owned())
	};

	let get_oldest = |queue: &Queue| -> Option<UserConnected> {
		if queue.is_empty() {
			return None;
		}

		let mut older = None;
		queue.iter().for_each(|(u, _)| {
			if older.is_none() {
				older = Some(u);
				return;
			}
			if older.unwrap().connected_at < u.connected_at {
				older = Some(u);
			}
		});
		older.map(|u| u.to_owned())
	};

	let socket = Arc::new(Mutex::new(socket));
	while let Some(Ok(msg)) = socket.clone().lock().await.recv().await {
		// let msg = match msg {
		// 	Ok(msg) => msg,
		// 	Err(err) => {
		// 		println!("Error: {}", err);
		// 		return;
		// 	}
		// };

		match msg {
			Message::Text(data) => {
				let Ok(req) = from_str::<Req>(&data) else { continue; };
				match req.r#type {
					ReqMessageType::Connection => {
						let Ok(data) = from_str::<ConData>(&req.data) else { continue };

						let Ok(id) = Uuid::from_str(&data.id) else {
							socket.lock().await.send(Message::Text(to_string(&Res {
								r#type: ResMessageType::Error,
								data: "invalid uuid".to_string()
							}).unwrap())).await.unwrap();
							continue;
						};

						for ice_candidate in state.ice_candidates.lock().await.iter() {
							socket
								.lock()
								.await
								.send(Message::Text(
									to_string(&Res {
										r#type: ResMessageType::IceCandidateAdded,
										data: to_string(ice_candidate).unwrap(),
									})
									.unwrap(),
								))
								.await
								.unwrap();
						}

						let now = Utc::now();

						if data.slave {
							let users = state.users_connected.lock().await;
							let Some(older) = get_oldest(&users) else {
								state.slaves_connected.lock().await.insert(UserConnected {
									id,
									connected_at: now,
								}, socket.clone());
								continue;
							};

							socket
								.lock()
								.await
								.send(Message::Text(
									to_string(&Res {
										r#type: ResMessageType::DidMatch,
										data: to_string(&QueueMatch {
											target_id: older.id,
										})
										.unwrap(),
									})
									.unwrap(),
								))
								.await
								.unwrap();
						}

						let slaves = state.slaves_connected.lock().await;
						let Some(older) = get_oldest(&slaves) else {
							state.users_connected.lock().await.insert(UserConnected {
								id,
								connected_at: now,
							}, socket.clone());
							continue;
						};

						socket
							.lock()
							.await
							.send(Message::Text(
								to_string(&Res {
									r#type: ResMessageType::DidMatch,
									data: to_string(&QueueMatch {
										target_id: older.id,
									})
									.unwrap(),
								})
								.unwrap(),
							))
							.await
							.unwrap();
					}

					ReqMessageType::WebRTCConRequest => {
						let Ok(data) = from_str::<WebRTCConReq>(&req.data) else { continue; };
						// let res = to_string(&CodeRes {
						// 	sender_id: data.lock().await.sender_id.clone(),
						// 	code: data.code,
						// })
						// .unwrap();

						let mut users = state.users_connected.lock().await;
						let mut slaves = state.slaves_connected.lock().await;

						let user_exist = get_target(&users, &data.sender_id).is_some();
						if user_exist {
							socket
								.lock()
								.await
								.send(Message::Text(
									to_string(&Res {
										r#type: ResMessageType::Error,
										data: "UUID already signed".to_string(),
									})
									.unwrap(),
								))
								.await
								.unwrap();
							continue;
						}

						let Some(target) = get_oldest(&slaves) else {
							users.insert(UserConnected { id: Uuid::from_str(&data.sender_id).unwrap(), connected_at: Utc::now() }, socket.clone()).unwrap();
							continue;
						};

						slaves
							.get_mut(&target)
							.unwrap()
							.lock()
							.await
							.send(Message::Text(
								to_string(&Res {
									r#type: ResMessageType::WebRTCConRequested,
									data: req.data,
								})
								.unwrap(),
							))
							.await
							.unwrap();

						// let slave_target = get_target(&slaves, &data.target_id);
						// match (user_target, slave_target) {
						// 	(Some(key), None) => users
						// 		.get_mut(&key)
						// 		.unwrap()
						// 		.lock().await.send(Message::Text(res))
						// 		.await
						// 		.unwrap(),
						// 	(None, Some(key)) => slaves
						// 		.get_mut(&key)
						// 		.unwrap()
						// 		.lock().await.send(Message::Text(res))
						// 		.await
						// 		.unwrap(),
						// 	(None, None) => socket
						// 		.lock().await.send(Message::Text(
						// 			r#" { "error": "usuário não encontrado!" }"#.to_string(),
						// 		))
						// 		.await
						// 		.unwrap(),
						// 	(Some(_), Some(_)) => unreachable!(),
						// }
					}

					ReqMessageType::WebRTCPeerConnectionEstablished => {
						let Ok(data) = from_str::<WebRTCPCEstablished>(&req.data) else { continue; };

						let mut users = state.users_connected.lock().await;
						let mut slaves = state.slaves_connected.lock().await;

						// FIXME: duplicated code
						let user_target = get_target(&users, &data.target_id);
						let slave_target = get_target(&slaves, &data.target_id);
						match (user_target, slave_target) {
							(Some(key), None) => {
								users.remove(&key).unwrap();
							}
							(None, Some(key)) => {
								slaves.remove(&key).unwrap();
							}
							(None, None) => {
								// since they can possibly not even be added to the queue, there's nothing to do here
							}
							(Some(_), Some(_)) => unreachable!(),
						};

						let user_target = get_target(&users, &data.sender_id);
						let slave_target = get_target(&slaves, &data.sender_id);
						match (user_target, slave_target) {
							(Some(key), None) => {
								users.remove(&key).unwrap();
							}
							(None, Some(key)) => {
								slaves.remove(&key).unwrap();
							}
							(None, None) => {
								// since they can possibly not even be added to the queue, there's nothing to do here
							}
							(Some(_), Some(_)) => unreachable!(),
						};
					}

					ReqMessageType::WebRTCOfferAnwsered => {
						let Ok(data) = from_str::<WebRTCAnswer>(&req.data) else { continue; };

						let res = to_string(&Res {
							r#type: ResMessageType::WebRTCAnswer,
							data: to_string(&data).unwrap(),
						})
						.unwrap();

						let mut users = state.users_connected.lock().await;
						let mut slaves = state.slaves_connected.lock().await;

						let user_target = get_target(&users, &data.target_id);
						let slave_target = get_target(&slaves, &data.target_id);
						match (user_target, slave_target) {
							(Some(key), None) => users
								.get_mut(&key)
								.unwrap()
								.lock()
								.await
								.send(Message::Text(res))
								.await
								.unwrap(),
							(None, Some(key)) => slaves
								.get_mut(&key)
								.unwrap()
								.lock()
								.await
								.send(Message::Text(res))
								.await
								.unwrap(),
							(None, None) => {
								// since they can possibly not even be added to the queue, there's nothing to do here
							}
							(Some(_), Some(_)) => unreachable!(),
						};
					}
					ReqMessageType::AddIceCandidate => {
						let Ok(data) = from_str::<IceReq>(&req.data) else { continue; };
						state
							.ice_candidates
							.lock()
							.await
							.push(data.candidate.clone());

						let res = Message::Text(
							to_string(&Res {
								r#type: ResMessageType::IceCandidateAdded,
								data: to_string(&data.candidate).unwrap(),
							})
							.unwrap(),
						);

						println!("sexo {:#?}", state.ice_candidates.lock().await);
						for (user, socket) in state.slaves_connected.lock().await.iter_mut() {
							if data.id == user.id.to_string() {
								continue;
							}
							socket.lock().await.send(res.clone()).await.unwrap();
						}
						for (user, socket) in state.users_connected.lock().await.iter_mut() {
							if data.id == user.id.to_string() {
								continue;
							}
							socket.lock().await.send(res.clone()).await.unwrap();
						}
					}
				}
			}

			Message::Binary(_) => {
				println!("ws: \nclient sent binary data");
			}
			Message::Ping(_) => {
				println!("ws: \nsocket ping");
			}
			Message::Pong(_) => {
				println!("ws: \nsocket pong");
			}

			Message::Close(data) => {
				// TODO: handle this properly
				let Some(data) = data else { return; };
				let Ok(id) = Uuid::from_str(&data.reason) else {
				socket.lock().await.send(Message::Text(r#"{ "error": "invalid uuid" }"#.to_string())).await.unwrap();
				return;
			};

				let state = state.clone();
				let users = state.users_connected.lock().await;
				if let Some((user, _)) = users.iter().find(|(u, _)| u.id.eq(&id)) {
					state.users_connected.lock().await.remove(user);
				};

				let slaves = state.slaves_connected.lock().await;
				if let Some((slave, _)) = slaves.iter().find(|(u, _)| u.id.eq(&id)) {
					state.slaves_connected.lock().await.remove(slave);
				};
			}
		}
	}
}
