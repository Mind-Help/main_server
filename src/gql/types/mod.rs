use async_graphql::{Error as GqlError, Result as GqlResult};
use tokio_postgres::Error;

pub mod date;
pub mod uuid;
pub mod input;

pub fn from_db_result<T>(db_result: Result<T, Error>) -> GqlResult<T> {
	match db_result {
		Ok(val) => Ok(val),
		Err(err) => Err(GqlError {
			message: format!("DATABASE ERROR: {}", err),
			extensions: None,
			source: None,
		}),
	}
}
