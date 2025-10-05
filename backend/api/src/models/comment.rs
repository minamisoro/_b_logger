use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::schema::comments;

#[derive(Queryable, Selectable, Serialize, Deserialize, Debug, Clone)]
#[diesel(table_name = comments)]
pub struct Comment {
    pub id: Uuid,
    pub post_id: Uuid,
    pub author_id: Uuid,
    pub parent_comment_id: Option<Uuid>,
    pub content: String,
    pub is_approved: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = comments)]
pub struct NewComment {
    pub post_id: Uuid,
    pub author_id: Uuid,
    pub parent_comment_id: Option<Uuid>,
    pub content: String,
    pub is_approved: bool,
}
