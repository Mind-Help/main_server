use std::{collections::HashMap, sync::Arc};

use axum::extract::ws::WebSocket;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;
use uuid::Uuid;

#[derive(Deserialize, Serialize)]
pub(super) enum ReqMessageType {
	Connection,
	WebRTCConRequest,
	WebRTCOfferAnwsered,
	WebRTCPeerConnectionEstablished,
	AddIceCandidate,
}

#[derive(Deserialize, Serialize)]
pub(super) enum ResMessageType {
	WebRTCConRequested,
	WebRTCAnswer,
	// WebRTCConAccepted,
	IceCandidateAdded,
	DidMatch,
	Error,
}

#[derive(Deserialize, Serialize)]
pub(super) struct Req {
	pub r#type: ReqMessageType,
	pub data: String,
}

#[derive(Deserialize, Serialize)]
pub(super) struct Res {
	pub r#type: ResMessageType,
	pub data: String,
}

#[derive(Deserialize, Serialize)]
pub(super) struct Code {
	pub sdp: String,
	pub r#type: String,
}

#[derive(Deserialize, Serialize)]
pub(super) struct WebRTCConReq {
	pub sender_id: String,
	pub target_id: String,
	pub code: Code,
}
#[derive(Deserialize, Serialize)]
pub(super) struct QueueMatch {
	pub target_id: Uuid,
}

#[derive(Deserialize, Serialize)]
pub(super) struct WebRTCPCEstablished {
	pub target_id: String,
	pub sender_id: String,
}

pub(super) type WebRTCAnswer = WebRTCConReq;

// #[derive(Deserialize, Serialize)]
// pub(super) struct CodeRes {
// 	pub sender_id: String,
// 	pub code: Code,
// }

#[derive(Deserialize, Serialize, Debug)]
pub(super) struct ConData {
	pub id: String,
	pub slave: bool,
}

// FIXME: poor design
#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub struct UserConnected {
	pub id: Uuid,
	pub connected_at: DateTime<Utc>,
}

#[derive(Deserialize, Serialize, Debug)]
pub(super) struct IceReq {
	pub id: String,
	pub candidate: RTCIceCandidate,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct RTCIceCandidate {
	candidate: String,
	sdp_mline_index: f32,
	spd_mid: String,
	username_fragment: String,
}

pub type Queue = HashMap<UserConnected, Arc<Mutex<WebSocket>>>;

pub struct AppState {
	pub users_connected: Mutex<Queue>,
	pub slaves_connected: Mutex<Queue>,
	pub ice_candidates: Mutex<Vec<RTCIceCandidate>>,
}
