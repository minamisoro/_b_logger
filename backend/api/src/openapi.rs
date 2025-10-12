use utoipa::OpenApi;

use crate::response::ApiError;
use crate::routes::timeline::types::{GetTimelineResponse, TimelinePost};
use crate::routes::user_groups::types::{
    UserGroupResponse, UserGroupMemberInfo, CreateUserGroupRequest,
    UpdateUserGroupRequest, AddMemberRequest, ListUserGroupsResponse
};
use crate::routes::users::types::UserInfo;

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::routes::timeline::get_timeline,
        crate::routes::user_groups::list_user_groups,
        crate::routes::user_groups::create_user_group,
        crate::routes::user_groups::update_user_group,
        crate::routes::user_groups::delete_user_group,
        crate::routes::user_groups::add_group_member,
        crate::routes::user_groups::remove_group_member,
        crate::routes::users::get_random_user,
    ),
    components(
        schemas(
            ApiError,
            TimelinePost,
            GetTimelineResponse,
            UserGroupResponse,
            UserGroupMemberInfo,
            CreateUserGroupRequest,
            UpdateUserGroupRequest,
            AddMemberRequest,
            ListUserGroupsResponse,
            UserInfo
        )
    ),
    tags(
        (name = "Timeline", description = "Timeline endpoints for retrieving published posts"),
        (name = "User Groups", description = "User group management endpoints"),
        (name = "Users", description = "User information endpoints")
    ),
    info(
        title = "Blogger API",
        version = "0.1.0",
        description = "REST API for the blog platform",
    )
)]
pub struct ApiDoc;
