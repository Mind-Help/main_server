use async_graphql::{EmptyMutation, EmptySubscription, Schema};

use crate::db::Database;

use super::query::Query;

pub type AppSchema = Schema<Query, EmptyMutation, EmptySubscription>;

pub async fn build_schema() -> AppSchema {
	let db = Database::new().await.unwrap();
	Schema::build(Query::default(), EmptyMutation, EmptySubscription)
		.data(db)
		.finish()
}
