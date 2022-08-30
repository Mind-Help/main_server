use async_graphql::MergedObject;

use doctors_query::DoctorsQuery;
use users_query::UserQuery;

mod doctors_query;
mod users_query;

#[derive(MergedObject, Default)]
pub struct Query(UserQuery, DoctorsQuery);
