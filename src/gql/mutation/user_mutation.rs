use async_graphql::{Context, Object, Result};

use crate::{
	db::{models::User, Database},
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
				data.phone,
				data.status,
			)
			.await,
		)
	}
	async fn delete_user(&self, ctx: &Context<'_>, id: String) -> Result<bool> {
		let db = ctx.data::<Database>().unwrap();
		from_db_result(db.delete_user(id).await)
	}
}
