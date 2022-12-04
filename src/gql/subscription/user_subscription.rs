use async_graphql::{
	futures_util::{Stream, StreamExt},
	Enum, Subscription, ID,
};

use crate::gql::types::simple_broker::SimpleBroker;

#[derive(Default)]
pub struct UserSubscription;

#[derive(Enum, PartialEq, Eq, Clone, Copy)]
enum MutationType {
	Connect,
	Request,
}

#[derive(Clone)]
struct ChatChanged {
	id: ID,
	mutation_type: MutationType,
}

/*#[Subscription]
impl UserSubscription {
	async fn chat(&self, mutation_type: Option<MutationType>) -> impl Stream<Item = ChatChanged> {
		SimpleBroker::<ChatChanged>::subscribe().filter(move |event| {
			let res = if let Some(mutation_type) = mutation_type {
				event.mutation_type == mutation_type
			} else {
				true
			};
			async move { res }
		})
	}
}*/
