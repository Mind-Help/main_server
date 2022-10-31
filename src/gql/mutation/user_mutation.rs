use async_graphql::{Context, Object, Result};

use crate::{
	db::{models::User, Database, Where, WhereFields},
	gql::types::{from_db_result, input::UserIT},
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
				data.password,
				"".to_string(),
				data.status.unwrap_or_default(),
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
		if let Ok(user) = db
			.get_user(Where {
				field: WhereFields::Email,
				value: email,
			})
			.await
		{
			if argon2::verify_encoded(&user.password, password.as_bytes()).unwrap() {
				return Ok(user);
			}
			return Err("senha incorreta".into());
		}
		return Err("usuario não encontrado".into());
	}
}
