use crate::WSClient;
use std::collections::HashMap;

use super::{
    utils::CHANNEL_PAIR_DELIMITER,
    ws_client_internal::{MiscMessage, WSClientInternal},
};
use log::*;
use serde_json::Value;

pub(super) const EXCHANGE_NAME: &str = "Bitfinex";

const WEBSOCKET_URL: &str = "wss://api-pub.bitfinex.com/ws/2";

/// The WebSocket client for Bitfinex, including all markets(<https://docs.bitfinex.com/docs/ws-general>).
pub struct BitfinexWSClient<'a> {
    client: WSClientInternal<'a>,
}

fn channel_to_command(channel: &str, subscribe: bool) -> String {
    let delim = channel.find(CHANNEL_PAIR_DELIMITER).unwrap();
    let ch = &channel[..delim];
    let symbol = &channel[(delim + 1)..];

    format!(
        r#"{{"event": "{}", "channel": "{}", "symbol": "{}"}}"#,
        if subscribe {
            "subscribe"
        } else {
            "unsubscribe"
        },
        ch,
        symbol
    )
}

fn serialize_command(channels: &[String], subscribe: bool) -> Vec<String> {
    channels
        .iter()
        .map(|s| channel_to_command(s, subscribe))
        .collect()
}

fn on_misc_msg(msg: &str) -> MiscMessage {
    let resp = serde_json::from_str::<HashMap<String, Value>>(&msg);
    if resp.is_err() {
        debug_assert!(msg.starts_with('['));
        return MiscMessage::Normal;
    }
    let obj = resp.unwrap();

    let event = obj.get("event").unwrap().as_str().unwrap();
    match event {
        "error" => error!("{} from {}", msg, EXCHANGE_NAME),
        "info" => info!("{} from {}", msg, EXCHANGE_NAME),
        "pong" => debug!("{} from {}", msg, EXCHANGE_NAME),
        "conf" => warn!("{} from {}", msg, EXCHANGE_NAME),
        "subscribed" => {
            // TODO: store channel_id in {"event":"subscribed","channel":"trades","chanId":676277,"symbol":"tBTCF0:USTF0","pair":"BTCF0:USTF0"}
            info!("{} from {}", msg, EXCHANGE_NAME);
        }
        "unsubscribed" => info!("{} from {}", msg, EXCHANGE_NAME),
        _ => (),
    }

    MiscMessage::Misc
}

define_client!(
    BitfinexWSClient,
    EXCHANGE_NAME,
    WEBSOCKET_URL,
    serialize_command,
    on_misc_msg
);

#[cfg(test)]
mod tests {
    #[test]
    fn test_spot_command() {
        assert_eq!(
            r#"{"event": "subscribe", "channel": "trades", "symbol": "tBTCUSD"}"#,
            super::channel_to_command("trades:tBTCUSD", true)
        );

        assert_eq!(
            r#"{"event": "unsubscribe", "channel": "trades", "symbol": "tBTCUSD"}"#,
            super::channel_to_command("trades:tBTCUSD", false)
        );
    }

    #[test]
    fn test_swap_command() {
        assert_eq!(
            r#"{"event": "subscribe", "channel": "trades", "symbol": "BTCF0:USTF0"}"#,
            super::channel_to_command("trades:BTCF0:USTF0", true)
        );

        assert_eq!(
            r#"{"event": "unsubscribe", "channel": "trades", "symbol": "BTCF0:USTF0"}"#,
            super::channel_to_command("trades:BTCF0:USTF0", false)
        );
    }
}
