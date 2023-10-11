#![allow(unused_parens)]
pub mod config;
pub mod file_watcher;
pub mod api;
pub mod multiplex_service;
pub mod grpc;

use std::{net::SocketAddr, sync::Arc};

use axum::{Router, routing::get};
use clap::Parser;
use config::manager::ConfigurationManager;
use env_logger::Env;
use tokio::sync::RwLock;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::{file_watcher::file_watcher::FileWatcher, api::{documentation::ApiDoc, rest::*}, proto::configuration_manager_server::ConfigurationManagerServer, grpc::grpc::ConfigurationManagerImpl, multiplex_service::multiplex_service::MultiplexService};

pub mod proto {
    tonic::include_proto!("configuration");

    pub(crate) const FILE_DESCRIPTOR_SET: &[u8] =
        tonic::include_file_descriptor_set!("configuration_descriptor");
}

/// Configuration management server
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to configuration file to load
    #[arg(short, long, default_value_t = ("example_config.yaml".to_string())) ]
    config_path: String,
}

#[tokio::main]
async fn main() {
    env_logger::Builder::from_env(Env::default().default_filter_or("debug")).target(env_logger::Target::Stdout).init();
    //tracing_subscriber::fmt::init();

    let args = Args::parse();

    log::info!("Starting configuration management server");

    let configuration_manager = ConfigurationManager::from_yaml_file(&args.config_path).unwrap();
    configuration_manager.save().unwrap();

    let shared_state = Arc::new(RwLock::new(configuration_manager));

    let app = Router::new()
    .route("/groups", get(list_groups))
    .route("/groups/:group_name", get(get_group))
    .route("/groups/:group_name/:param_name", get(get_config_param))
    .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
    .with_state(Arc::clone(&shared_state));

    let reflection_service = tonic_reflection::server::Builder::configure()
    .register_encoded_file_descriptor_set(proto::FILE_DESCRIPTOR_SET)
    .build()
    .unwrap();

    let grpc = tonic::transport::Server::builder()
    .add_service(reflection_service)
    .add_service(ConfigurationManagerServer::new(ConfigurationManagerImpl { shared_state: Arc::clone(&shared_state) }))
    .into_service();

    let service = MultiplexService::new(app, grpc);

    let server = axum::Server::bind(&SocketAddr::from(([127, 0, 0, 1], 3000)))
    .serve(tower::make::Shared::new(service));

    let watcher_task = {
        let mut config_watcher = FileWatcher::new(shared_state.read().await.get_db_file_path()).await.unwrap();
        tokio::spawn(async move {
            loop {
                match config_watcher.watch().await {
                    Ok(event) => {
                        if event.unwrap().kind.is_access() {
                            shared_state.write().await.reload_config().unwrap();
                            log::info!("Configuration reloaded")
                        }
                    },
                    Err(e) => {
                        log::error!("Error watching config file: {:?}", e);
                    }
                }
            }
        })
    };

    tokio::select! {
        _ = server => {
            log::error!("Server exited unexpectedly");
        }
        _ = watcher_task => {
            log::error!("Watcher task exited unexpectedly");
        }
    }
}