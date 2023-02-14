mod grpc_server;

pub mod trading_settings_integration_grpc {
    tonic::include_proto!("trading_settings_integration");
}

pub use grpc_server::*;