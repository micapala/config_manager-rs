use std::sync::Arc;

use tokio::sync::RwLock;
use tonic::{Response, Request, Status};

use crate::{proto::{configuration_manager_server::*, UpdateGroupRequest, UpdateGroupResponse, ReadGroupResponse, ReadGroupRequest, CreateGroupResponse, CreateGroupRequest, DeleteGroupRequest, GetAllGroupsRequest, GetAllGroupsResponse, DeleteGroupResponse}, config::{group::ConfigurationGroup, self}};

#[derive(Default)]
pub struct ConfigurationManagerImpl {
    pub shared_state: Arc<RwLock<crate::config::ConfigurationManager>>,
}

#[tonic::async_trait]
impl ConfigurationManager for ConfigurationManagerImpl {
    async fn create_group(
        &self,
        request: Request<CreateGroupRequest>,
    ) -> Result<Response<CreateGroupResponse>, Status> {
        let req = request.into_inner();
        let group_name = req.group_name.clone();
        let mut config_manager = self.shared_state.write().await;

        log::info!("Creating group: {}", &group_name);

        config_manager
            .configuration_groups
            .add_group(group_name.clone(), ConfigurationGroup::default());

        Ok(Response::new(CreateGroupResponse { group_name }))
    }

    async fn read_group(
        &self,
        request: Request<ReadGroupRequest>,
    ) -> Result<Response<ReadGroupResponse>, Status> {
        let req = request.into_inner();
        let config_manager = self.shared_state.read().await;

        match config_manager.configuration_groups.get_group(&req.group_name) {
            Some(group) => Ok(Response::new(ReadGroupResponse {
                group: Some(group.clone().into()),
            })),
            None => {
                log::error!("Group not found: {}", &req.group_name);
                Err(Status::not_found("Group not found"))
            }
        }
    }

    async fn update_group(
        &self,
        request: Request<UpdateGroupRequest>,
    ) -> Result<Response<UpdateGroupResponse>, Status> {
        let req = request.into_inner();

        let mut config_manager = self.shared_state.write().await;
        match config_manager.configuration_groups.groups.get_mut(&req.group_name) {
            Some(group) => {
                group.set_entries(req.group.as_ref().unwrap().entries.clone());
                log::info!("Updating group: {}", &req.group_name);
                return Ok(Response::new(UpdateGroupResponse {
                    group: Some(req.group.unwrap().clone().into()),
                }))
            },
            None => return Err(Status::not_found("Group not found")),
        }
    }

    async fn delete_group(
        &self,
        request: Request<DeleteGroupRequest>,
    ) -> Result<Response<DeleteGroupResponse>, Status> {
        let req = request.into_inner();
        let group_name = req.group_name.clone();
        let mut config_manager = self.shared_state.write().await;

        match config_manager.configuration_groups.remove_group(&group_name) {
            Some(_) => {
                log::info!("Deleting group: {}", &group_name);
                Ok(Response::new(DeleteGroupResponse { group_name }))
            },
            None => {
                log::error!("Group not found: {}", &group_name);
                Err(Status::not_found("Group not found"))
            },
        }
    }

    async fn get_all_groups(
        &self,
        _request: Request<GetAllGroupsRequest>,
    ) -> Result<Response<GetAllGroupsResponse>, Status> {
        let config_manager = self.shared_state.read().await;

        Ok(Response::new(GetAllGroupsResponse {
            groups: Some(config_manager.configuration_groups.clone().into()),
        }))
    }
}