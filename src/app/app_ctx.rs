use std::sync::Arc;

use my_no_sql_tcp_reader::{MyNoSqlDataReader, MyNoSqlTcpConnection};
use my_nosql_contracts::{
    TradingGroupNoSqlEntity, TradingInstrumentNoSqlEntity, TradingProfileNoSqlEntity,
};
use rust_extensions::AppStates;

use crate::SettingsModel;

pub const APP_VERSION: &'static str = env!("CARGO_PKG_VERSION");
pub const APP_NAME: &'static str = env!("CARGO_PKG_NAME");

pub struct AppContext {
    pub instruments_ns_reader: Arc<MyNoSqlDataReader<TradingInstrumentNoSqlEntity>>,
    pub trading_profiles_ns_reader: Arc<MyNoSqlDataReader<TradingProfileNoSqlEntity>>,
    pub trading_groups_ns_reader: Arc<MyNoSqlDataReader<TradingGroupNoSqlEntity>>,
    pub settings: Arc<SettingsModel>,
    pub app_states: Arc<AppStates>,
    pub ns_connection: MyNoSqlTcpConnection,
}

impl AppContext {
    pub async fn new(settings: &Arc<SettingsModel>) -> AppContext {
        let ns_connection = MyNoSqlTcpConnection::new(APP_NAME.to_string(), settings.clone());

        return AppContext {
            instruments_ns_reader: ns_connection.get_reader().await,
            trading_profiles_ns_reader: ns_connection.get_reader().await,
            trading_groups_ns_reader: ns_connection.get_reader().await,
            app_states: Arc::new(AppStates::create_initialized()),
            settings: settings.clone(),
            ns_connection
        };
    }
}
