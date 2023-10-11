use utoipa::OpenApi;
use super::rest::*;


#[derive(OpenApi)]
#[openapi(
    paths(
        list_groups,
        get_group,
        get_config_param
    ),
    tags(
        (name = "config", description = "Configuration manager api")
    )
)]
pub struct ApiDoc;