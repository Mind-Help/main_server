use argon2::{hash_encoded, Config};
use async_graphql::{Enum, SimpleObject};
use chrono::Utc;
use redis::FromRedisValue;
use serde::{Deserialize, Serialize};
use serde_json::from_str;
use uuid::Uuid;

use std::{env, io::Read};

use crate::gql::types::{date::DateTimeUtc, uuid::MyUuid};

#[derive(Enum, Serialize, Deserialize, Copy, Clone, Eq, PartialEq)]
pub enum UserStatus {
	Normal,
	Pro,
}

impl ToString for UserStatus {
	fn to_string(&self) -> String {
		match self {
			Self::Normal => "NORMAL".to_string(),
			Self::Pro => "PRO".to_string(),
		}
	}
}

#[derive(SimpleObject, Serialize, Deserialize)]
pub struct User {
	pub id: MyUuid,
	pub name: String,
	pub email: String,
	pub password: String,
	pub phone: String,
	pub status: UserStatus,
	pub docs_history: Vec<String>,
	pub resp_doc_id: Option<String>,
	pub created_at: DateTimeUtc,
	pub updated_at: DateTimeUtc,
}

impl FromRedisValue for User {
	fn from_byte_vec(vec: &[u8]) -> Option<Vec<Self>> {
		if vec.len() <= 0 {
			return None;
		}
		let mut buf = String::new();
		vec.clone().read_to_string(&mut buf).unwrap();
		Some(from_str(&buf).unwrap())
	}
	fn from_redis_value(v: &redis::Value) -> redis::RedisResult<Self> {
		todo!()
	}
	fn from_redis_values(items: &[redis::Value]) -> redis::RedisResult<Vec<Self>> {
		todo!()
	}
}

impl User {
	pub fn new(
		name: String,
		email: String,
		password: String,
		phone: String,
		status: UserStatus,
	) -> Self {
		let salt = env!("PASSWD_SECRET");
		let password =
			hash_encoded(password.as_bytes(), salt.as_bytes(), &Config::default()).unwrap();
		Self {
			id: MyUuid(Uuid::new_v4()),
			name,
			email,
			password,
			phone,
			status,
			docs_history: Vec::new(),
			resp_doc_id: None,
			created_at: DateTimeUtc(Utc::now()),
			updated_at: DateTimeUtc(Utc::now()),
		}
	}
}

#[derive(SimpleObject, Serialize)]
pub struct Doctor {
	pub id: MyUuid,
	pub name: String,
	pub email: String,
	pub password: String,
	pub phone: String,
	pub resume: String,
	pub patients_history: Vec<String>,
	pub created_at: DateTimeUtc,
	pub updated_at: DateTimeUtc,
}

impl Doctor {
	pub fn new(
		name: String,
		email: String,
		password: String,
		phone: String,
		resume: String,
	) -> Self {
		let salt = env!("PASSWD_SECRET");
		let password =
			hash_encoded(password.as_bytes(), salt.as_bytes(), &Config::default()).unwrap();
		Self {
			id: MyUuid(Uuid::new_v4()),
			name,
			email,
			password,
			phone,
			resume,
			patients_history: Vec::new(),
			created_at: DateTimeUtc(Utc::now()),
			updated_at: DateTimeUtc(Utc::now()),
		}
	}
}
