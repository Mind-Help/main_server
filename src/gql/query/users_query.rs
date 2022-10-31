use async_graphql::{Context, Object, Result};

use crate::{
	db::{models::User, Database, Where},
	gql::types::from_db_result,
};

#[derive(Default)]
pub struct UserQuery;

#[Object]
impl UserQuery {
	async fn get_users(&self, ctx: &Context<'_>) -> Result<Vec<User>> {
		let db = ctx.data::<Database>().unwrap();
		from_db_result(db.get_users().await)
	}
	async fn get_user(&self, ctx: &Context<'_>, r#where: Where) -> Result<User> {
		let db = ctx.data::<Database>().unwrap();
		from_db_result(db.get_user(r#where).await)
	}
}
