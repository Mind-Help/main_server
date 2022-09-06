use std::fmt::Display;

use redis::{Client, JsonAsyncCommands, RedisResult};

use models::*;

pub mod models;

#[derive(Debug)]
struct DbError {
	message: String,
}

impl Display for DbError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		Ok(f.write_str(&format!("{}\n", self.message)).unwrap())
	}
}

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
		password: String,
		phone: String,
		status: UserStatus,
	) -> RedisResult<User> {
		let user = User::new(name, email, password, phone, status);

		let user = self
			.client
			.get_tokio_connection()
			.await?
			.json_arr_append("users", "$", &user)
			.await?;

		Ok(user)
	}
	pub async fn get_user(&self, id: String) -> RedisResult<User> {
		let mut con = self.client.get_tokio_connection().await?;

		// TODO: incomplete
		let user: User = redis::cmd("FT.SEARCH")
			.arg(id)
			.query_async(&mut con)
			.await?;

		Ok(user)
	}
	pub async fn get_users(&self) -> RedisResult<Vec<User>> {
		let mut con = self.client.get_tokio_connection().await?;
		Ok(con.json_get("users", "$").await?)
	}
	pub async fn update_user(&self) -> RedisResult<User> {
		todo!()
	}
	pub async fn delete_user(&self, _id: String) -> RedisResult<bool> {
		todo!()
	}
	pub async fn create_doctor(
		&self,
		name: String,
		email: String,
		password: String,
		phone: String,
		resume: String,
	) -> RedisResult<Doctor> {
		Ok(Doctor::new(name, email, password, phone, resume))
	}
	pub async fn get_doctor(&self, _id: String) -> RedisResult<Doctor> {
		todo!()
	}
	pub async fn get_doctors(&self) -> RedisResult<Vec<Doctor>> {
		todo!()
	}
	pub async fn update_doctor(&self) -> RedisResult<Doctor> {
		todo!()
	}
	pub async fn delete_doctor(&self, _id: String) -> RedisResult<bool> {
		todo!()
	}
}
