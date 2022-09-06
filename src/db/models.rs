use std::{io::Read, str::from_utf8};

use argon2::{hash_encoded, Config};
use async_graphql::{Enum, SimpleObject};
use axum::body::HttpBody;
use chrono::Utc;
use redis::{FromRedisValue, RedisError, Value};
use serde::{ser::Error, Deserialize, Serialize};
use serde_json::from_str;
use uuid::Uuid;

use crate::gql::types::{date::DateTimeUtc, uuid::MyUuid};

#[derive(Enum, Serialize, Deserialize, Copy, Clone, Eq, PartialEq)]
pub enum UserStatus {
	Normal,
	Pro,
}

impl ToString for UserStatus {
	fn to_string(&self) -> String {
		match self {
			Self::Normal => String::from("NORMAL"),
			Self::Pro => String::from("PRO"),
		}
	}
}

#[derive(SimpleObject, Serialize, Deserialize, Clone)]
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

// FIXME
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
		match v {
			Value::Nil => todo!(),
			Value::Int(int) => Err(RedisError::from(serde_json::Error::custom(format!(
				"int??: {int}"
			)))),
			Value::Data(data) => {
				let mut buffer = String::new();
				data.as_slice().read_to_string(&mut buffer).unwrap();

				if let Ok(user) = from_str(&buffer) {
					return Ok(user);
				}

				Err(RedisError::from(serde_json::Error::custom(
					"unable to parse value from redis",
				)))
			}
			Value::Bulk(bulk) => {
				let pseudo_users = bulk
					.iter()
					.map(|val| User::from_redis_value(val).unwrap())
					.collect::<Vec<User>>();

				Ok(pseudo_users[0].clone())
			}
			Value::Status(_) => unreachable!(),
			Value::Okay => unreachable!(),
		}
	}
	fn from_redis_values(items: &[redis::Value]) -> redis::RedisResult<Vec<Self>> {
		Ok(items
			.iter()
			.map(|val| User::from_redis_value(val).unwrap())
			.collect())
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
