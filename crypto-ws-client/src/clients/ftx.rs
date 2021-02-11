use crate::{WSClient};
use std::collections::{HashMap, VecDeque};
use tungstenite::Message;

use super::{
    utils::{CHANNEL_PAIR_DELIMITER},
    ws_client_internal::{MiscMessage, WSClientInternal},
};
use log::*;
use serde_json::Value;

pub(super) const EXCHANGE_NAME: &str = "FTX";

const WEBSOCKET_URL: &str = "wss://ftx.com/ws/";

/// The WebSocket client for FTX (<https://docs.ftx.com/#websocket-api>).
pub struct FTXWSClient<'a> {
    client: WSClientInternal<'a>,
}

fn name_pairs_to_command(super_channel: String, subscribe: bool) -> std::string::String{
    let v: Vec<&str> = super_channel.split(CHANNEL_PAIR_DELIMITER).collect();
    format!(
        r#"{{"op": "{}", "channel": "{}", "market": "{}"}}"#,
        if subscribe {
            "subscribe"
        } else {
            "unsubscribe"
        },
        v[0].to_string(),
        v[1].to_string(),
    )
}

fn channels_to_commands(channels: &[String], subscribe: bool) -> Vec<String> {
    channels
        .iter()
        .map(|ch| name_pairs_to_command(ch.to_string(), subscribe))
        .collect()
}

fn on_misc_msg(msg: &str) -> MiscMessage {
    let resp = serde_json::from_str::<Value>(&msg);
    if resp.is_err() {
        error!("{} is not a JSON string, {}", msg, EXCHANGE_NAME);
        return MiscMessage::Misc;
    }
    let obj = resp.unwrap();

//    match obj["channel"] {
//        "ticker" => pairs.push(pair.to_string()),
//    }
    MiscMessage::Normal
}


define_client!(
    FTXWSClient,
    EXCHANGE_NAME,
    WEBSOCKET_URL,
    channels_to_commands,
    on_misc_msg
);

#[cfg(test)]
mod tests {
    #[test]
    fn test_one_channel() {
        //assert_eq!(1, 1);
        let commands = super::channels_to_commands(&vec!["trades:BTC-PERP".to_string()],true);
        assert_eq!(1, commands.len());
        assert_eq!(r#"{"op": "subscribe", "channel": "trades", "market": "BTC-PERP"}"#, commands[0]);
    }
}
