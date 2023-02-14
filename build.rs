fn main() {
    tonic_build::compile_protos("proto/trading_settings_integration_grpc_service.proto").unwrap();
}
