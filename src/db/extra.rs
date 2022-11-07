use std::io::Read;

use async_graphql::{Enum, InputObject};
use chrono::{DateTime, Utc};
use redis::{aio::Connection, RedisResult, Value};
use redis_graph::{AsyncGraphCommands, GraphResultSet, GraphValue};
use uuid::Uuid;

use super::models::{User, UserStatus};

impl Default for UserStatus {
	fn default() -> Self {
		Self::Normal
	}
}

impl ToString for UserStatus {
	fn to_string(&self) -> String {
		match self {
			Self::Normal => String::from("NORMAL"),
			Self::Pro => String::from("PRO"),
		}
	}
}

impl From<String> for UserStatus {
	fn from(val: String) -> Self {
		if val.to_lowercase() == "pro" {
			return Self::Pro;
		}
		Self::Normal
	}
}

#[derive(Enum, PartialEq, Eq, Clone, Copy)]
pub enum WhereFields {
	Id,
	Email,
	Phone,
}

impl ToString for WhereFields {
	fn to_string(&self) -> String {
		match self {
			WhereFields::Id => "id".to_string(),
			WhereFields::Email => "email".to_string(),
			WhereFields::Phone => "phone".to_string(),
		}
	}
}

#[derive(InputObject)]
pub struct Where {
	pub field: WhereFields,
	pub value: String,
}

impl From<GraphValue> for User {
	fn from(val: GraphValue) -> Self {
		let extract_key = |key| {
			let GraphValue::Node(val) = &val else { return None; };
			let Some(Value::Data(data)) = val.properties.get(key) else { return  None;};

			let mut buffer = String::new();
			data.as_slice().read_to_string(&mut buffer).unwrap();

			Some(buffer)
		};

		User {
			id: Uuid::parse_str(&extract_key("id").unwrap()).unwrap(),
			name: extract_key("name").unwrap(),
			email: extract_key("email").unwrap(),
			password: extract_key("password"),
			phone: extract_key("phone"),
			photo: extract_key("photo"),
			resume: extract_key("resume"),
			status: extract_key("status").unwrap().into(),
			created_at: extract_key("created_at")
				.unwrap()
				.parse::<DateTime<Utc>>()
				.unwrap(),
			updated_at: extract_key("updated_at")
				.unwrap()
				.parse::<DateTime<Utc>>()
				.unwrap(),
		}
	}
}

impl From<GraphResultSet> for User {
	fn from(val: GraphResultSet) -> Self {
		// println!("{val:#?}");
		val.data[0].data.get("u").unwrap().to_owned().into()
	}
}

pub async fn create_user_query(user: &User, conn: &mut Connection) -> RedisResult<GraphResultSet> {
	let extract_to_null = |data: &Option<String>| {
		if let Some(data) = data {
			return "\"".to_string() + data + "\"";
		}
		"NULL".to_string()
	};
	conn.graph_query(
		"users",
		format!(
			r#"
			CREATE (u:User {{
				id: "{}",
				name: "{}",
				password: {},
				email: "{}",
				phone: {},
				photo: {},
				status: "{}",
				resume: NULL,
				created_at: "{}",
				updated_at: "{}"
			}}) RETURN u
		"#,
			user.id,
			user.name,
			extract_to_null(&user.password),
			user.email,
			extract_to_null(&user.phone),
			extract_to_null(&user.photo),
			user.status.to_string(),
			// extract_to_null(&user.resume),
			user.created_at,
			user.updated_at
		),
	)
	.await
}
