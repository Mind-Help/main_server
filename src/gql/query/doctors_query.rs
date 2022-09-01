use async_graphql::{Context, Object, Result};

use crate::{
	db::{models::Doctor, Database},
	gql::types::from_db_result,
};

#[derive(Default)]
pub struct DoctorsQuery;

#[Object]
impl DoctorsQuery {
	async fn get_doctors(&self, ctx: &Context<'_>) -> Result<Vec<Doctor>> {
		let db = ctx.data::<Database>().unwrap();
		from_db_result(db.get_doctors().await)
	}
	async fn get_doctor(&self, ctx: &Context<'_>, id: String) -> Result<Doctor> {
		let db = ctx.data::<Database>().unwrap();
		from_db_result(db.get_doctor(id).await)
	}
}
