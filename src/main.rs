use std::sync::Arc;
use trading_settings_integration::{start_grpc_server, AppContext, SettingsReader};

#[tokio::main]
async fn main() {
    let settings_reader = SettingsReader::new(".my-cfd").await;
    let settings_reader = Arc::new(settings_reader.get_settings().await);

    let app = Arc::new(AppContext::new(&settings_reader).await);

    app.ns_connection.start(my_logger::LOGGER.clone()).await;
    start_grpc_server(app.clone(), 8888);
    app.app_states.wait_until_shutdown().await;
}
