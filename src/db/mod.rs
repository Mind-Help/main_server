use std::env::var;
use std::str::FromStr;

use models::*;
use tokio_postgres::{Client, Config, Error, NoTls};

pub mod models;

pub struct Database {
	client: Client,
}

impl Database {
	pub async fn new() -> Result<Self, Error> {
		let db_config = Config::from_str(
			var("DATABASE_URL")
				.expect("$DATABASE_URL not found")
				.as_str(),
		)?;

		let (client, connection) = db_config.connect(NoTls).await?;

		tokio::spawn(async move {
			if let Err(e) = connection.await {
				eprintln!("connection error: {}", e);
			}
		});

		Ok(Self { client })
	}
	pub async fn create_user(
		&self,
		name: String,
		email: String,
		password: String,
		phone: String,
		status: UserStatus,
	) -> Result<User, Error> {
		Ok(User::new(name, email, password, phone, status))
	}
	pub async fn get_user(&self, id: String) -> Result<User, Error> {
		todo!()
	}
	pub async fn get_users(&self) -> Result<Vec<User>, Error> {
		todo!()
	}
	pub async fn update_user(&self) -> Result<User, Error> {
		todo!()
	}
	pub async fn delete_user(&self, id: String) -> Result<(), Error> {
		todo!()
	}
	pub async fn create_doctor(
		&self,
		name: String,
		email: String,
		password: String,
		phone: String,
		resume: String,
	) -> Result<Doctor, Error> {
		Ok(Doctor::new(name, email, password, phone, resume))
	}
	pub async fn get_doctor(&self, id: String) -> Result<Doctor, Error> {
		todo!()
	}
	pub async fn get_doctors(&self) -> Result<Vec<Doctor>, Error> {
		todo!()
	}
	pub async fn update_doctor(&self) -> Result<Doctor, Error> {
		todo!()
	}
	pub async fn delete_doctor(&self, id: String) -> Result<(), Error> {
		todo!()
	}
}
