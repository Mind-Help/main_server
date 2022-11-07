use argon2::{hash_encoded, Config};
use async_graphql::{Enum, SimpleObject};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Enum, Serialize, Deserialize, Copy, Clone, Eq, PartialEq)]
pub enum UserStatus {
	Normal,
	Pro,
}

#[derive(SimpleObject, Serialize, Deserialize, Clone)]
pub struct User {
	pub id: Uuid,
	pub name: String,
	pub email: String,
	#[graphql(skip)]
	pub password: Option<String>,
	pub phone: Option<String>,
	pub photo: Option<String>,
	pub resume: Option<String>,
	pub status: UserStatus,
	pub created_at: DateTime<Utc>,
	pub updated_at: DateTime<Utc>,
}

impl User {
	pub fn new(
		name: String,
		email: String,
		password: Option<String>,
		phone: Option<String>,
		photo: Option<String>,
		resume: Option<String>,
		status: UserStatus,
	) -> Self {
		let salt = env!("PASSWD_SECRET");
		let password = password.map(|password| {
			hash_encoded(password.as_bytes(), salt.as_bytes(), &Config::default()).unwrap()
		});

		Self {
			id: Uuid::new_v4(),
			name,
			email,
			password,
			phone,
			photo,
			resume,
			status,
			created_at: Utc::now(),
			updated_at: Utc::now(),
		}
	}
}
