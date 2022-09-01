use async_graphql::{Context, Object, Result};

use crate::{
	db::{models::Doctor, Database},
	gql::types::{from_db_result, input::DoctorIT},
};

#[derive(Default)]
pub struct DoctorMutation;

#[Object]
impl DoctorMutation {
	async fn create_doctor(&self, ctx: &Context<'_>, data: DoctorIT) -> Result<Doctor> {
		let db = ctx.data::<Database>().unwrap();
		from_db_result(
			db.create_doctor(
				data.name,
				data.email,
				data.password,
				data.phone,
				data.resume,
			)
			.await,
		)
	}
	async fn delete_doctor(&self, ctx: &Context<'_>, id: String) -> Result<bool> {
		let db = ctx.data::<Database>().unwrap();
		from_db_result(db.delete_doctor(id).await)
	}
}
