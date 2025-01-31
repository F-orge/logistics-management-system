use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, types::Uuid};

#[derive(FromRow)]
pub struct StorageModel {
    id: Uuid,
    name: String,
    r#type: String,
    size: i64,
    owner_id: Option<Uuid>,
}
