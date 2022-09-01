use argon2::{hash_encoded, Config};
use async_graphql::{Enum, SimpleObject};
use chrono::Utc;
use std::env;
use uuid::Uuid;

use crate::gql::types::{date::DateTimeUtc, uuid::MyUuid};

#[derive(Enum, Copy, Clone, Eq, PartialEq)]
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

#[derive(SimpleObject)]
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

#[derive(SimpleObject)]
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
