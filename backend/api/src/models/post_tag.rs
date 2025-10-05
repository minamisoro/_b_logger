use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::schema::post_tags;

#[derive(Queryable, Selectable, Serialize, Deserialize, Debug, Clone)]
#[diesel(table_name = post_tags)]
pub struct PostTag {
    pub post_id: Uuid,
    pub tag_id: Uuid,
    pub created_at: NaiveDateTime,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = post_tags)]
pub struct NewPostTag {
    pub post_id: Uuid,
    pub tag_id: Uuid,
}
