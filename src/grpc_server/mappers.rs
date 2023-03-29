use my_nosql_contracts::{
    TradingGroupNoSqlEntity, TradingInstrumentDayOff, TradingInstrumentNoSqlEntity,
    TradingProfileInstrument, TradingProfileNoSqlEntity,
};

use crate::trading_settings_integration_grpc::{
    TradingGroupGrpcModel, TradingInstrumentGrpcModel, TradingInstrumentDayOffGrpcModel,
    TradingProfileGrpcModel, TradingProfileInstrumentGrpcModel,
};

impl Into<TradingInstrumentGrpcModel> for TradingInstrumentNoSqlEntity {
    fn into(self) -> TradingInstrumentGrpcModel {
        TradingInstrumentGrpcModel {
            id: self.row_key,
            name: self.name,
            digits: self.digits as u32,
            base: self.base,
            quote: self.quote,
            tick_size: self.tick_size,
            trading_disabled: self.trading_disabled,
            trading_instrument_day_offs: self
                .days_off
                .iter()
                .map(|x| x.to_owned().into())
                .collect(),
        }
    }
}

impl Into<TradingInstrumentDayOffGrpcModel> for TradingInstrumentDayOff {
    fn into(self) -> TradingInstrumentDayOffGrpcModel {
        TradingInstrumentDayOffGrpcModel {
            day_from: self.dow_from as u32,
            time_from: self.time_from,
            day_to: self.dow_to as u32,
            time_to: self.time_to,
        }
    }
}

impl Into<TradingProfileGrpcModel> for TradingProfileNoSqlEntity {
    fn into(self) -> TradingProfileGrpcModel {
        TradingProfileGrpcModel {
            id: self.row_key,
            margin_call_percent: self.margin_call_percent,
            stop_out_percent: self.stop_out_percent,
            trading_profile_instruments: self
                .instruments
                .iter()
                .map(|x| x.to_owned().into())
                .collect(),
        }
    }
}

impl Into<TradingProfileInstrumentGrpcModel> for TradingProfileInstrument {
    fn into(self) -> TradingProfileInstrumentGrpcModel {

        let so = match self.stop_out_percent{
            Some(src) => src,
            None => 0.0,
        };

        TradingProfileInstrumentGrpcModel {
            id: self.id,
            min_operation_volume: self.min_operation_volume as u64,
            max_operation_volume: self.max_operation_volume as u64,
            max_position_volume: self.max_position_volume as u64,
            open_position_min_delay_ms: self.open_position_min_delay_ms as u64,
            open_position_max_delay_ms: self.open_position_max_delay_ms as u64,
            tp_slippage: false,
            sl_slippage: false,
            is_trending: false,
            open_position_slippage: false,
            leverages: self.leverages.iter().map(|x| *x as u32).collect(),
            stop_out_percent: so,
        }
    }
}

impl Into<TradingGroupGrpcModel> for TradingGroupNoSqlEntity {
    fn into(self) -> TradingGroupGrpcModel {
        TradingGroupGrpcModel {
            id: self.id,
            name: self.name,
            trading_profile_id: self.trading_profile_id,
            trading_disabled: self.trading_disabled,
        }
    }
}
