use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::schema::post_links;

#[derive(Queryable, Selectable, Serialize, Deserialize, Debug, Clone)]
#[diesel(table_name = post_links)]
pub struct PostLink {
    pub id: Uuid,
    pub source_post_id: Uuid,
    pub target_post_id: Uuid,
    pub created_at: NaiveDateTime,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = post_links)]
pub struct NewPostLink {
    pub source_post_id: Uuid,
    pub target_post_id: Uuid,
}
