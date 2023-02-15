use std::time::Duration;

use tokio::time::sleep;
use trading_settings_integration::start_grpc_server;

#[tokio::main]
async fn main() {
    start_grpc_server(8888);
    
    loop {
        sleep(Duration::from_secs(100)).await;
    }
}