use async_graphql::{EmptySubscription, Schema};

use crate::db::Database;

use super::{mutation::Mutation, query::Query};

pub type AppSchema = Schema<Query, Mutation, EmptySubscription>;

pub async fn build_schema() -> AppSchema {
	let db = Database::new().await.unwrap();
	Schema::build(Query::default(), Mutation::default(), EmptySubscription)
		.data(db)
		.finish()
}
