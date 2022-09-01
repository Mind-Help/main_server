use async_graphql::InputObject;

use crate::db::models::UserStatus;

#[derive(InputObject)]
pub struct UserIT {
	pub name: String,
	pub email: String,
	pub password: String,
	pub phone: String,
	pub status: UserStatus,
}

#[derive(InputObject)]
pub struct DoctorIT {
	pub name: String,
	pub email: String,
	pub password: String,
	pub phone: String,
	pub resume: String,
}
