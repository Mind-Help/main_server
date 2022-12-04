use std::{collections::HashMap, str::FromStr};

use async_graphql::{Enum, InputObject};
use uuid::Uuid;

#[derive(InputObject)]
pub struct UserIT {
	pub name: String,
	// #[graphql(validator(email))]
	pub email: String,
	pub password: String,
	// pub status: Option<UserStatus>,
}

#[derive(InputObject)]
pub struct GoogleUserIT {
	pub name: String,
	pub email: String,
	pub photo: Option<String>,
	pub password: String,
	// pub status: UserStatus,
}

#[derive(InputObject)]
pub struct UpdateUserIT {
	pub id: Uuid,
	pub data: HashMap<Fields, String>,
}

#[derive(Enum, PartialEq, Eq, Clone, Copy, Hash)]
pub enum Fields {
	Name,
	Email,
	Password,
	Resume,
	Phone,
	Status,
}

impl ToString for Fields {
	fn to_string(&self) -> String {
		match self {
			Fields::Name => "name".to_string(),
			Fields::Email => "email".to_string(),
			Fields::Password => "password".to_string(),
			Fields::Resume => "resume".to_string(),
			Fields::Phone => "phone".to_string(),
			Fields::Status => "status".to_string(),
		}
	}
}

impl FromStr for Fields {
	type Err = String;
	fn from_str(s: &str) -> Result<Self, Self::Err> {
		if s.to_lowercase() == "name" {
			return Ok(Fields::Name);
		}
		if s.to_lowercase() == "email" {
			return Ok(Fields::Email);
		}
		if s.to_lowercase() == "password" {
			return Ok(Fields::Password);
		}
		if s.to_lowercase() == "resume" {
			return Ok(Fields::Resume);
		}
		if s.to_lowercase() == "phone" {
			return Ok(Fields::Phone);
		}
		if s.to_lowercase() == "status" {
			return Ok(Fields::Status);
		}
		Err("unable to parse str".to_string())
	}
}
