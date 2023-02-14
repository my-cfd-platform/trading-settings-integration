FROM rust:slim
COPY ./target/release/trading-settings-integration ./target/release/trading-settings-integration
ENTRYPOINT ["./target/release/trading-settings-integration"]