use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::error::Error;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[schema(as = PaginatedResult<T>)]
pub struct PaginatedResult<T> {
    pub total: u32,
    pub items: T,
}
