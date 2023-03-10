mod grpc_server;
mod app;

pub mod trading_settings_integration_grpc {
    tonic::include_proto!("trading_settings_integration");
}

pub use grpc_server::*;
pub use app::*;