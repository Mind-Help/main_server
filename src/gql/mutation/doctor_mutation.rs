use async_graphql::{Context, Object, Result};

use crate::{
	db::{models::User, Database, Where},
	gql::types::from_db_result,
};

#[derive(Default)]
pub struct DoctorMutation;

#[Object]
impl DoctorMutation {
	async fn create_doctor(&self, _ctx: &Context<'_>, _data: String) -> Result<User> {
		/*let db = ctx.data::<Database>().unwrap();
		from_db_result(
			db.create_doctor(
				data.name,
				data.email,
				data.password,
				"".to_string(),
				data.resume,
			)
			.await,
		)*/
		todo!()
	}
	async fn delete_doctor(&self, ctx: &Context<'_>, r#where: Where) -> Result<bool> {
		let db = ctx.data::<Database>().unwrap();
		from_db_result(db.delete_doctor(r#where).await)
	}
}
