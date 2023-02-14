use std::pin::Pin;

use rust_extensions::date_time::DateTimeAsMicroseconds;

use crate::{
    trading_settings_integration_grpc::{
        trading_settings_integration_grpc_service_server::TradingSettingsIntegrationGrpcService,
        PingResponse, TradingGroupGrpcModel, TradingInstrumentGrpcModel, TradingProfileGrpcModel,
    },
    GrpcService,
};

#[tonic::async_trait]
impl TradingSettingsIntegrationGrpcService for GrpcService {
    type GetTradingInstumentsStream = Pin<
        Box<
            dyn tonic::codegen::futures_core::Stream<
                    Item = Result<TradingInstrumentGrpcModel, tonic::Status>,
                > + Send
                + Sync
                + 'static,
        >,
    >;

    type GetTradingProfilesStream = Pin<
        Box<
            dyn tonic::codegen::futures_core::Stream<
                    Item = Result<TradingProfileGrpcModel, tonic::Status>,
                > + Send
                + Sync
                + 'static,
        >,
    >;

    type GetTradingGroupsStream = Pin<
        Box<
            dyn tonic::codegen::futures_core::Stream<
                    Item = Result<TradingGroupGrpcModel, tonic::Status>,
                > + Send
                + Sync
                + 'static,
        >,
    >;

    async fn get_trading_instuments(
        &self,
        _: tonic::Request<()>,
    ) -> Result<tonic::Response<Self::GetTradingInstumentsStream>, tonic::Status> {
        return my_grpc_extensions::grpc_server::send_vec_to_stream(vec![], |itm| itm).await;
    }

    async fn get_trading_profiles(
        &self,
        _: tonic::Request<()>,
    ) -> Result<tonic::Response<Self::GetTradingProfilesStream>, tonic::Status> {
        return my_grpc_extensions::grpc_server::send_vec_to_stream(vec![], |itm| itm).await;
    }

    async fn get_trading_groups(
        &self,
        _: tonic::Request<()>,
    ) -> Result<tonic::Response<Self::GetTradingGroupsStream>, tonic::Status> {
        return my_grpc_extensions::grpc_server::send_vec_to_stream(vec![], |itm| itm).await;
    }

    async fn ping(
        &self,
        _: tonic::Request<()>,
    ) -> Result<tonic::Response<PingResponse>, tonic::Status> {
        return Ok(tonic::Response::new(PingResponse {
            service_name: "TRADING_SETTIGNS_INTEGRATION".to_string(),
            date_time: DateTimeAsMicroseconds::now().unix_microseconds as u64,
        }));
    }
}
