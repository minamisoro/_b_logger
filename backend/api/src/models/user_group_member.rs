use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::schema::user_group_members;

#[derive(Queryable, Selectable, Serialize, Deserialize, Debug, Clone)]
#[diesel(table_name = user_group_members)]
pub struct UserGroupMember {
    pub id: Uuid,
    pub group_id: Uuid,
    pub user_id: Uuid,
    pub created_at: NaiveDateTime,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = user_group_members)]
pub struct NewUserGroupMember {
    pub group_id: Uuid,
    pub user_id: Uuid,
}
