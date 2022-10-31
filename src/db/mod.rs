use redis::{Client, RedisError, RedisResult};
use redis_graph::*;

use models::*;
use serde::de::Error;

mod extra;
pub mod models;
pub use extra::{Where, WhereFields};

use extra::create_user_query;

pub struct Database {
	client: Client,
}

// TODO: remove the json feature + better error handling
impl Database {
	pub async fn new() -> RedisResult<Self> {
		let client = Client::open(env!("REDIS_URL"))?;
		Ok(Self { client })
	}

	pub async fn create_user(
		&self,
		name: String,
		email: String,
		password: String,
		phone: String,
		status: UserStatus,
	) -> RedisResult<User> {
		let user = User::new(name, email, password, phone, status);

		let res = create_user_query(&user, &mut self.client.get_tokio_connection().await?).await?;
		// println!("{:#?}", res.metadata);
		if res.metadata[0] == "Nodes created: 1" {
			return Ok(user);
		}

		Err(RedisError::from(serde_json::Error::custom(
			"database internal error",
		)))
	}

	pub async fn get_user(&self, r#where: Where) -> RedisResult<User> {
		let res = self
			.client
			.get_tokio_connection()
			.await?
			.graph_ro_query(
				"users",
				format!(
					"MATCH (u: User) WHERE u.{} = \"{}\" RETURN u",
					r#where.field.to_string(),
					r#where.value
				),
			)
			.await?;
		Ok(res.into())
	}

	pub async fn get_users(&self) -> RedisResult<Vec<User>> {
		let res = self
			.client
			.get_tokio_connection()
			.await?
			.graph_ro_query("users", "MATCH (u: User) RETURN u")
			.await?;
		Ok(res
			.data
			.into_iter()
			.map(|item| item.data.get("u").unwrap().to_owned().into())
			.collect::<Vec<User>>())
	}

	pub async fn update_user(&self) -> RedisResult<User> {
		todo!()
	}

	pub async fn delete_user(&self, r#where: Where) -> RedisResult<bool> {
		let res = self
			.client
			.get_tokio_connection()
			.await?
			.graph_query(
				"users",
				format!(
					"MATCH (u: User) WHERE u.{} = \"{}\" DETACH DELETE u",
					r#where.field.to_string(),
					r#where.value.to_string()
				),
			)
			.await?;
		if res.metadata.len() <= 0 {
			return Ok(true);
		}
		Err(RedisError::from(serde_json::Error::custom(
			"user not found",
		)))
	}

	/*pub async fn create_doctor(
		&self,
		name: String,
		email: String,
		password: String,
		phone: String,
		resume: String,
	) -> RedisResult<Doctor> {
		Ok(Doctor::new(name, email, password, phone, resume))
	}*/

	pub async fn get_doctor(&self, r#where: Where) -> RedisResult<User> {
		let res = self
			.client
			.get_tokio_connection()
			.await?
			.graph_ro_query(
				"users",
				format!(
					"MATCH (u: User) WHERE u.{} = \"{}\" AND u.resume IS NOT NULL RETURN u",
					r#where.field.to_string(),
					r#where.value
				),
			)
			.await?;
		Ok(res.into())
	}

	pub async fn get_doctors(&self) -> RedisResult<Vec<User>> {
		let res = self
			.client
			.get_tokio_connection()
			.await?
			.graph_ro_query(
				"users",
				"MATCH (u: User) WHERE u.resume IS NOT NULL RETURN u",
			)
			.await?;
		Ok(res
			.data
			.into_iter()
			.map(|item| item.data.get("u").unwrap().to_owned().into())
			.collect::<Vec<User>>())
	}

	pub async fn delete_doctor(&self, r#where: Where) -> RedisResult<bool> {
		let res = self
			.client
			.get_tokio_connection()
			.await?
			.graph_query(
				"users",
				format!(
					"MATCH (u: User) WHERE u.{} = \"{}\" DETACH DELETE u",
					r#where.field.to_string(),
					r#where.value.to_string()
				),
			)
			.await?;
		if res.metadata.len() <= 0 {
			return Ok(true);
		}
		Err(RedisError::from(serde_json::Error::custom(
			"doctor not found",
		)))
	}
}
