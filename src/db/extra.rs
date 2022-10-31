use std::io::Read;

use async_graphql::{Enum, InputObject};
use chrono::{DateTime, Utc};
use redis::{aio::Connection, FromRedisValue, RedisResult, Value};
use redis_graph::{AsyncGraphCommands, GraphResultSet, GraphValue};
use serde_json::from_str;
use uuid::Uuid;

use super::models::{Doctor, User, UserStatus};

impl Default for UserStatus {
	fn default() -> Self {
		Self::NORMAL
	}
}

impl ToString for UserStatus {
	fn to_string(&self) -> String {
		match self {
			Self::NORMAL => String::from("NORMAL"),
			Self::PRO => String::from("PRO"),
		}
	}
}

impl From<String> for UserStatus {
	fn from(val: String) -> Self {
		if val.to_lowercase() == "pro" {
			return Self::PRO;
		}
		Self::NORMAL
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
		let extract_key = |key: &str| {
			if let GraphValue::Node(x) = &val {
				if let Value::Data(y) = x.properties.get(key).unwrap() {
					let mut buffer = String::new();
					y.as_slice().read_to_string(&mut buffer).unwrap();
					return buffer;
				};
			}
			todo!()
		};
		User {
			id: Uuid::parse_str(&extract_key("id")).unwrap(),
			name: extract_key("name"),
			email: extract_key("email"),
			password: extract_key("password"),
			phone: "".to_string(),
			status: extract_key("status").into(),
			created_at: extract_key("created_at").parse::<DateTime<Utc>>().unwrap(),
			updated_at: extract_key("updated_at").parse::<DateTime<Utc>>().unwrap(),
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
	conn.graph_query(
		"users",
		format!(
			r#"
			CREATE (u:User {{
				id: "{}",
				name: "{}",
				password: "{}",
				email: "{}",
				phone: NULL,
				status: "{}",
				resume: NULL,
				created_at: "{}",
				updated_at: "{}"
			}}) RETURN u
		"#,
			user.id,
			user.name,
			user.password,
			user.email,
			user.status.to_string(),
			user.created_at.to_string(),
			user.updated_at.to_string()
		),
	)
	.await
}
