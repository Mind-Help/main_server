use async_graphql::InputObject;

// use crate::db::models::UserStatus;

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
