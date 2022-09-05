use async_graphql::{Error as GqlError, Result as GqlResult};
use redis::RedisError;

pub mod date;
pub mod input;
pub mod uuid;

pub fn from_db_result<T>(db_result: Result<T, RedisError>) -> GqlResult<T> {
	match db_result {
		Ok(val) => Ok(val),
		Err(err) => Err(GqlError {
			message: format!("DATABASE ERROR: {}", err),
			extensions: None,
			source: None,
		}),
	}
}
