use async_graphql::{Object, Result};

#[derive(Default)]
pub struct MainQuery;

#[Object]
impl MainQuery {
    async fn hello(&self) -> Result<String> {
        Ok("hello".to_owned())
    }
}
