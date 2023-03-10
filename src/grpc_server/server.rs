use std::{net::SocketAddr, sync::Arc};
use tonic::transport::Server;

use crate::{trading_settings_integration_grpc::trading_settings_integration_grpc_service_server::TradingSettingsIntegrationGrpcServiceServer, AppContext};

#[derive(Clone)]
pub struct GrpcService {
    pub app: Arc<AppContext>,
}

impl GrpcService {
    pub fn new(app: Arc<AppContext>) -> Self {
        Self { app }
    }
}

pub fn start_grpc_server(app: Arc<AppContext>, port: u16) {
    let addr = SocketAddr::from(([0, 0, 0, 0], port));

    let service = GrpcService::new(app);

    println!("Listening to {:?} as grpc endpoint", addr);

    tokio::spawn(async move {
        Server::builder()
            .add_service(TradingSettingsIntegrationGrpcServiceServer::new(
                service.clone(),
            ))
            .serve(addr)
            .await
    });
}
