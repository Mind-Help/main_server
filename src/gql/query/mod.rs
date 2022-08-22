use async_graphql::MergedObject;
use test_query::MainQuery;

mod test_query;

#[derive(MergedObject, Default)]
pub struct Query(MainQuery);
