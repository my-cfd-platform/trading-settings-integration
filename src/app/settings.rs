use my_no_sql_tcp_reader::MyNoSqlTcpConnectionSettings;
use my_settings_reader::SettingsModel;
use serde_derive::{Serialize, Deserialize};

#[derive(SettingsModel, Serialize, Deserialize, Debug, Clone)]
pub struct SettingsModel{
    #[serde(rename = "NoSqlTcp")]
    pub no_sql_tcp: String,
}

#[async_trait::async_trait]
impl MyNoSqlTcpConnectionSettings for SettingsModel {
    async fn get_host_port(&self) -> String{
        self.no_sql_tcp.clone()
    }
}