use std::pin::Pin;

use my_nosql_contracts::{
    TradingGroupNoSqlEntity, TradingInstrumentNoSqlEntity, TradingProfileNoSqlEntity,
};
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
        let instruments = self
            .app
            .instruments_ns_reader
            .get_by_partition_key_as_vec(TradingInstrumentNoSqlEntity::generate_partition_key())
            .await;

        let instruments = match instruments {
            Some(instruments) => instruments
                .iter()
                .map(|x| x.as_ref().to_owned().into())
                .collect(),
            None => vec![],
        };
        return my_grpc_extensions::grpc_server::send_vec_to_stream(instruments, |itm| itm).await;
    }

    async fn get_trading_profiles(
        &self,
        _: tonic::Request<()>,
    ) -> Result<tonic::Response<Self::GetTradingProfilesStream>, tonic::Status> {
        let tps = self
            .app
            .trading_profiles_ns_reader
            .get_by_partition_key_as_vec(TradingProfileNoSqlEntity::generate_partition_key())
            .await;

        let tps = match tps {
            Some(tps) => tps.iter().map(|x| x.as_ref().to_owned().into()).collect(),
            None => vec![],
        };

        return my_grpc_extensions::grpc_server::send_vec_to_stream(tps, |itm| itm).await;
    }

    async fn get_trading_groups(
        &self,
        _: tonic::Request<()>,
    ) -> Result<tonic::Response<Self::GetTradingGroupsStream>, tonic::Status> {
        let tgs = self
            .app
            .trading_groups_ns_reader
            .get_by_partition_key_as_vec(TradingGroupNoSqlEntity::generate_partition_key())
            .await;

        let tgs = match tgs {
            Some(tgs) => tgs.iter().map(|x| x.as_ref().to_owned().into()).collect(),
            None => vec![],
        };
        return my_grpc_extensions::grpc_server::send_vec_to_stream(tgs, |itm| itm).await;
    }

    async fn ping(
        &self,
        _: tonic::Request<()>,
    ) -> Result<tonic::Response<PingResponse>, tonic::Status> {
        return Ok(tonic::Response::new(PingResponse {
            service_name: "TRADING_SETTINGS_INTEGRATION".to_string(),
            date_time: DateTimeAsMicroseconds::now().unix_microseconds as u64,
        }));
    }
}
