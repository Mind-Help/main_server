use async_graphql::MergedObject;

mod user_subscription;

#[derive(MergedObject, Default)]
pub struct Subscription();
