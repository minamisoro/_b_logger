use axum::{
    extract::{FromRef, FromRequestParts},
    http::{StatusCode, request::Parts},
};
use diesel::PgConnection;
use diesel::r2d2::{self, ConnectionManager, PooledConnection};

use crate::response::{ApiError, IntoFailure};

pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;
pub struct DbConnection(pub PooledConnection<ConnectionManager<PgConnection>>);

#[derive(thiserror::Error, Debug)]
#[error("Failed to connect to database: {0}")]
pub struct DbConnectionError(String);

impl IntoFailure for DbConnectionError {
    const CODE: &str = "DATABASE_CONNECTION_ERROR";
    const STATUS_CODE: StatusCode = StatusCode::INTERNAL_SERVER_ERROR;
}

pub fn establish_connection_pool(database_url: &str) -> DbPool {
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create database pool")
}

impl<S> FromRequestParts<S> for DbConnection
where
    DbPool: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = ApiError;

    async fn from_request_parts(_parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let pool = DbPool::from_ref(state);

        pool.get()
            .map(DbConnection)
            .map_err(|err| DbConnectionError(err.to_string()).into_api_error())
    }
}
