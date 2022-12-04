//
// FIXME:
// "SQL Injection" for each query
//

use redis::{Client, ErrorKind, RedisError, RedisResult};
use redis_graph::*;

use models::*;

mod extra;
pub mod models;
pub use extra::{Where, WhereFields};

use extra::build_create_user_query;

use crate::gql::types::input::UpdateUserIT;

use self::extra::build_update_user_query;

pub struct Database {
	client: Client,
}

impl Database {
	pub async fn new() -> RedisResult<Self> {
		let client = Client::open(env!("REDIS_URL"))?;
		Ok(Self { client })
	}

	pub async fn create_user(
		&self,
		name: String,
		email: String,
		phone: Option<String>,
		photo: Option<String>,
		password: Option<String>,
	) -> RedisResult<User> {
		let user = User::new(
			name,
			email,
			password,
			phone,
			photo,
			None,
			UserStatus::Normal,
		);

		let res =
			build_create_user_query(&user, &mut self.client.get_tokio_connection().await?).await?;

		println!("{res:#?}");
		// if res.metadata[0] == "Nodes created: 1" {
		// 	return Ok(user);
		// }

		Err(RedisError::from((
			ErrorKind::TryAgain,
			"internal server error",
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

		if res.data.is_empty() {
			return Err(RedisError::from((ErrorKind::TryAgain, "user not found")));
		}

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

	pub async fn update_user(&self, input: UpdateUserIT) -> RedisResult<User> {
		let res =
			build_update_user_query(input, &mut self.client.get_tokio_connection().await?).await?;
		if res.data.is_empty() {
			return Err(RedisError::from((ErrorKind::TryAgain, "user not found")));
		}
		Ok(res.into())
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
					r#where.value
				),
			)
			.await?;

		if res.metadata[0] == "Nodes deleted: 1" {
			return Ok(true);
		}
		Err(RedisError::from((ErrorKind::TryAgain, "user not found")))
	}

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

		if res.data.is_empty() {
			return Err(RedisError::from((ErrorKind::TryAgain, "doctor not found")));
		}

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
					"MATCH (u: User) WHERE u.{} = \"{}\" AND u.resume IS NOT NULL DETACH DELETE u",
					r#where.field.to_string(),
					r#where.value
				),
			)
			.await?;

		if res.metadata.is_empty() {
			return Ok(true);
		}

		Err(RedisError::from((ErrorKind::TryAgain, "doctor not found")))
	}
}
