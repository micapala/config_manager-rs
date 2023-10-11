use axum::{extract::{State, Path}, http::StatusCode, Json};

use crate::config::{SharedConfigManager, configuration::ConfigurationGroups, group::ConfigurationGroup};



#[utoipa::path(
    get,
    path = "/groups",
    responses(
        (status = 200, description = "List all configuration groups", body = [crate::config_manager::config::ConfigurationGroups])
    )
)]
pub async fn list_groups(State(state): axum::extract::State<SharedConfigManager>) -> (StatusCode, Json<ConfigurationGroups>) {
    let groups = state.read().await.configuration_groups.clone();
    (StatusCode::OK, Json(groups))
}

#[utoipa::path(
    get,
    path = "/groups/{group_name}",
    params(
        ("group_name", description="Name of the group to get")
    ),
    responses(
        (status = 200, description = "Get group successfully", body = [crate::config_manager::group::ConfigurationGroup]),
        (status = 404, description = "Group not found", body = [default::Default])
    )
)]
pub async fn get_group(
    Path(group_name): Path<String>,
    State(state): axum::extract::State<SharedConfigManager>
) -> (StatusCode, Json<ConfigurationGroup>) {
    let group_name = group_name;
    log::info!("Getting group: {}", group_name);
    let groups = state.read().await.configuration_groups.clone();
    let group = groups.get_group(&group_name);
    match group {
        Some(gr) => (StatusCode::OK, Json(gr.to_owned())),
        None => (StatusCode::NOT_FOUND, Json(ConfigurationGroup::default())),
    }
}

#[utoipa::path(
    get,
    path = "/groups/{group_name}/{param_name}",
    params(
        ("group_name", description="Name of the group to get"),
        ("param_name", description="Name of the parameter to get"),
    ),
    responses(
        (status = 200, description = "Get group successfully", body = [String]),
        (status = 404, description = "Group not found", body = [default::Default])
    )
)]
pub async fn get_config_param(
    Path((group_name, param_name)): Path<(String, String)>,
    State(state): axum::extract::State<SharedConfigManager>
) -> (StatusCode, Json<String>) {
    log::info!("Getting param {param_name} from group {group_name}");
    let groups = state.read().await.configuration_groups.clone();
    let group = groups.get_group(&group_name);
    match group {
        Some(gr) => {
            let param = gr.entries.get(&param_name);
            match param {
                Some(p) => (StatusCode::OK, Json(p.to_owned())),
                None => (StatusCode::NOT_FOUND, Json("".to_string())),
            }
        },
        None => (StatusCode::NOT_FOUND, Json("".to_string())),
    }
}