use redis::{Client, JsonAsyncCommands, RedisResult};

use models::*;

pub mod models;

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
		let x = self
			.client
			.get_tokio_connection()
			.await?
			.json_set::<_, _, _, User>(user.id.0.to_string(), false, &user)
			.await?;
		Ok(x)
	}
	pub async fn get_user(&self, id: String) -> RedisResult<User> {
		todo!()
	}
	pub async fn get_users(&self) -> RedisResult<Vec<User>> {
		todo!()
	}
	pub async fn update_user(&self) -> RedisResult<User> {
		todo!()
	}
	pub async fn delete_user(&self, id: String) -> RedisResult<bool> {
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
	pub async fn get_doctor(&self, id: String) -> RedisResult<Doctor> {
		todo!()
	}
	pub async fn get_doctors(&self) -> RedisResult<Vec<Doctor>> {
		todo!()
	}
	pub async fn update_doctor(&self) -> RedisResult<Doctor> {
		todo!()
	}
	pub async fn delete_doctor(&self, id: String) -> RedisResult<bool> {
		todo!()
	}
}
