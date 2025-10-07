use axum::extract::Path;
use axum::{Json, Router, routing::{get, post, put, delete}};
use diesel::prelude::*;
use uuid::Uuid;

use crate::db::{DbConnection, DbPool};
use crate::response::{ApiError, ApiResponse};

use types::*;
pub mod types {
    use axum::http::StatusCode;
    use serde::{Deserialize, Serialize};
    use utoipa::ToSchema;
    use uuid::Uuid;

    use crate::response::IntoSuccess;

    #[derive(Serialize, ToSchema)]
    pub struct UserGroupResponse {
        pub id: String,
        pub user_id: String,
        pub name: String,
        pub members: Vec<UserGroupMemberInfo>,
        pub created_at: String,
        pub updated_at: String,
    }

    #[derive(Serialize, ToSchema)]
    pub struct UserGroupMemberInfo {
        pub user_id: String,
        pub username: String,
        pub display_name: Option<String>,
    }

    #[derive(Deserialize, ToSchema)]
    pub struct CreateUserGroupRequest {
        pub name: String,
    }

    #[derive(Deserialize, ToSchema)]
    pub struct UpdateUserGroupRequest {
        pub name: String,
    }

    #[derive(Deserialize, ToSchema)]
    pub struct AddMemberRequest {
        pub user_id: Uuid,
    }

    #[derive(Serialize, ToSchema)]
    pub struct ListUserGroupsResponse {
        pub groups: Vec<UserGroupResponse>,
    }

    impl IntoSuccess for UserGroupResponse {
        const STATUS_CODE: StatusCode = StatusCode::OK;
    }

    impl IntoSuccess for ListUserGroupsResponse {
        const STATUS_CODE: StatusCode = StatusCode::OK;
    }
}

/// List user's groups
#[utoipa::path(
    get,
    path = "/api/user-groups",
    responses(
        (status = 200, description = "Successfully retrieved user groups", body = ListUserGroupsResponse),
        (status = 500, description = "Database error", body = ApiError)
    ),
    tag = "User Groups"
)]
async fn list_user_groups(
    DbConnection(mut conn): DbConnection,
) -> ApiResponse<ListUserGroupsResponse> {
    use crate::models::{UserGroup, UserGroupMember, User};
    use crate::schema::{user_groups, user_group_members, users};

    // TODO: Get current user ID from auth context
    // For now, returning all groups
    let groups_data: Vec<UserGroup> = match user_groups::table
        .load(&mut conn)
    {
        Ok(groups) => groups,
        Err(e) => {
            return ApiResponse::Failure(ApiError {
                status_code: axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                code: "DATABASE_ERROR".to_string(),
                message: e.to_string(),
            });
        }
    };

    let mut groups_response = Vec::new();

    for group in groups_data {
        // Fetch members for each group
        let members: Vec<(UserGroupMember, User)> = match user_group_members::table
            .filter(user_group_members::group_id.eq(group.id))
            .inner_join(users::table.on(user_group_members::user_id.eq(users::id)))
            .load(&mut conn)
        {
            Ok(members) => members,
            Err(e) => {
                return ApiResponse::Failure(ApiError {
                    status_code: axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                    code: "DATABASE_ERROR".to_string(),
                    message: e.to_string(),
                });
            }
        };

        let member_info: Vec<UserGroupMemberInfo> = members
            .into_iter()
            .map(|(_, user)| UserGroupMemberInfo {
                user_id: user.id.to_string(),
                username: user.username,
                display_name: user.display_name,
            })
            .collect();

        groups_response.push(UserGroupResponse {
            id: group.id.to_string(),
            user_id: group.user_id.to_string(),
            name: group.name,
            members: member_info,
            created_at: group.created_at.and_utc().to_rfc3339(),
            updated_at: group.updated_at.and_utc().to_rfc3339(),
        });
    }

    ApiResponse::Success(ListUserGroupsResponse {
        groups: groups_response,
    })
}

/// Create a new user group
#[utoipa::path(
    post,
    path = "/api/user-groups",
    request_body = CreateUserGroupRequest,
    responses(
        (status = 200, description = "Successfully created user group", body = UserGroupResponse),
        (status = 500, description = "Database error", body = ApiError)
    ),
    tag = "User Groups"
)]
async fn create_user_group(
    DbConnection(mut conn): DbConnection,
    Json(req): Json<CreateUserGroupRequest>,
) -> ApiResponse<UserGroupResponse> {
    use crate::models::NewUserGroup;
    use crate::schema::user_groups;

    // TODO: Get current user ID from auth context
    // For now, using a placeholder UUID
    let current_user_id = Uuid::nil();

    use crate::models::UserGroup;

    let new_group = NewUserGroup {
        user_id: current_user_id,
        name: req.name,
    };

    let inserted_group: UserGroup = match diesel::insert_into(user_groups::table)
        .values(&new_group)
        .get_result(&mut conn)
    {
        Ok(group) => group,
        Err(e) => {
            return ApiResponse::Failure(ApiError {
                status_code: axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                code: "DATABASE_ERROR".to_string(),
                message: e.to_string(),
            });
        }
    };

    ApiResponse::Success(UserGroupResponse {
        id: inserted_group.id.to_string(),
        user_id: inserted_group.user_id.to_string(),
        name: inserted_group.name,
        members: vec![],
        created_at: inserted_group.created_at.and_utc().to_rfc3339(),
        updated_at: inserted_group.updated_at.and_utc().to_rfc3339(),
    })
}

/// Update a user group
#[utoipa::path(
    put,
    path = "/api/user-groups/{id}",
    params(
        ("id" = String, Path, description = "User group ID")
    ),
    request_body = UpdateUserGroupRequest,
    responses(
        (status = 200, description = "Successfully updated user group", body = UserGroupResponse),
        (status = 500, description = "Database error", body = ApiError)
    ),
    tag = "User Groups"
)]
async fn update_user_group(
    DbConnection(mut conn): DbConnection,
    Path(id): Path<Uuid>,
    Json(req): Json<UpdateUserGroupRequest>,
) -> ApiResponse<UserGroupResponse> {
    use crate::models::UserGroup;
    use crate::schema::user_groups;

    let updated_group: UserGroup = match diesel::update(user_groups::table.find(id))
        .set(user_groups::name.eq(req.name))
        .get_result(&mut conn)
    {
        Ok(group) => group,
        Err(e) => {
            return ApiResponse::Failure(ApiError {
                status_code: axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                code: "DATABASE_ERROR".to_string(),
                message: e.to_string(),
            });
        }
    };

    ApiResponse::Success(UserGroupResponse {
        id: updated_group.id.to_string(),
        user_id: updated_group.user_id.to_string(),
        name: updated_group.name,
        members: vec![], // TODO: Fetch members
        created_at: updated_group.created_at.and_utc().to_rfc3339(),
        updated_at: updated_group.updated_at.and_utc().to_rfc3339(),
    })
}

/// Delete a user group
#[utoipa::path(
    delete,
    path = "/api/user-groups/{id}",
    params(
        ("id" = String, Path, description = "User group ID")
    ),
    responses(
        (status = 200, description = "Successfully deleted user group"),
        (status = 500, description = "Database error", body = ApiError)
    ),
    tag = "User Groups"
)]
async fn delete_user_group(
    DbConnection(mut conn): DbConnection,
    Path(id): Path<Uuid>,
) -> Result<axum::http::StatusCode, ApiError> {
    use crate::schema::user_groups;

    match diesel::delete(user_groups::table.find(id))
        .execute(&mut conn)
    {
        Ok(_) => Ok(axum::http::StatusCode::NO_CONTENT),
        Err(e) => Err(ApiError {
            status_code: axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            code: "DATABASE_ERROR".to_string(),
            message: e.to_string(),
        }),
    }
}

/// Add member to user group
#[utoipa::path(
    post,
    path = "/api/user-groups/{id}/members",
    params(
        ("id" = String, Path, description = "User group ID")
    ),
    request_body = AddMemberRequest,
    responses(
        (status = 200, description = "Successfully added member"),
        (status = 500, description = "Database error", body = ApiError)
    ),
    tag = "User Groups"
)]
async fn add_group_member(
    DbConnection(mut conn): DbConnection,
    Path(group_id): Path<Uuid>,
    Json(req): Json<AddMemberRequest>,
) -> Result<axum::http::StatusCode, ApiError> {
    use crate::models::NewUserGroupMember;
    use crate::schema::user_group_members;

    let new_member = NewUserGroupMember {
        group_id,
        user_id: req.user_id,
    };

    match diesel::insert_into(user_group_members::table)
        .values(&new_member)
        .execute(&mut conn)
    {
        Ok(_) => Ok(axum::http::StatusCode::CREATED),
        Err(e) => Err(ApiError {
            status_code: axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            code: "DATABASE_ERROR".to_string(),
            message: e.to_string(),
        }),
    }
}

/// Remove member from user group
#[utoipa::path(
    delete,
    path = "/api/user-groups/{id}/members/{user_id}",
    params(
        ("id" = String, Path, description = "User group ID"),
        ("user_id" = String, Path, description = "User ID to remove")
    ),
    responses(
        (status = 200, description = "Successfully removed member"),
        (status = 500, description = "Database error", body = ApiError)
    ),
    tag = "User Groups"
)]
async fn remove_group_member(
    DbConnection(mut conn): DbConnection,
    Path((group_id, user_id)): Path<(Uuid, Uuid)>,
) -> Result<axum::http::StatusCode, ApiError> {
    use crate::schema::user_group_members;

    match diesel::delete(
        user_group_members::table
            .filter(user_group_members::group_id.eq(group_id))
            .filter(user_group_members::user_id.eq(user_id))
    )
    .execute(&mut conn)
    {
        Ok(_) => Ok(axum::http::StatusCode::NO_CONTENT),
        Err(e) => Err(ApiError {
            status_code: axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            code: "DATABASE_ERROR".to_string(),
            message: e.to_string(),
        }),
    }
}

pub fn routes() -> Router<DbPool> {
    Router::new()
        .route("/", get(list_user_groups).post(create_user_group))
        .route("/{id}", put(update_user_group).delete(delete_user_group))
        .route("/{id}/members", post(add_group_member))
        .route("/{id}/members/{user_id}", delete(remove_group_member))
}
