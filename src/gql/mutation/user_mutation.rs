use async_graphql::{Context, Object, Result};

use crate::{
	db::{models::User, Database, Where, WhereFields},
	gql::types::{
		from_db_result,
		input::{GoogleUserIT, UpdateUserIT, UserIT},
	},
};

#[derive(Default)]
pub struct UserMutation;

#[Object]
impl UserMutation {
	async fn create_user(&self, ctx: &Context<'_>, data: UserIT) -> Result<User> {
		let db = ctx.data::<Database>().unwrap();
		from_db_result(
			db.create_user(
				data.name,
				data.email,
				None,
				None,
				Some(data.password),
				// data.status.unwrap_or_default(),
			)
			.await,
		)
	}
	async fn delete_user(&self, ctx: &Context<'_>, r#where: Where) -> Result<bool> {
		let db = ctx.data::<Database>().unwrap();
		from_db_result(db.delete_user(r#where).await)
	}
	async fn login(&self, ctx: &Context<'_>, email: String, password: String) -> Result<User> {
		let db = ctx.data::<Database>().unwrap();
		let Ok(user) = db
			.get_user(Where {
				field: WhereFields::Email,
				value: email,
			})
			.await else { return Err("usuario não encontrado".into()); };

		if user.password.is_none() {
			return Err("senha não definida para este usuário".into());
		}
		if argon2::verify_encoded(&user.password.clone().unwrap(), password.as_bytes()).unwrap() {
			return Ok(user);
		}
		Err("senha incorreta".into())
	}
	async fn sign_up_google(&self, ctx: &Context<'_>, data: GoogleUserIT) -> Result<User> {
		let db = ctx.data::<Database>().unwrap();
		Ok(db
			.create_user(data.name, data.email, None, data.photo, None)
			.await?)
	}
	async fn update_user(&self, ctx: &Context<'_>, data: UpdateUserIT) -> Result<User> {
		let db = ctx.data::<Database>().unwrap();
		from_db_result(db.update_user(data).await)
	}
}
