use crypto_ws_client::{WSClient, FTXWSClient, BinanceFuturesWSClient};
use serde_json::Value;
use std::time::{SystemTime, UNIX_EPOCH};
use std::thread;

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

fn ftx_on_msg(message_string: String){
    let resp = serde_json::from_str::<Value>(&message_string.to_string());
    if resp.is_err() {
        println!("{} is not a JSON string", message_string);
    }
    let obj = resp.unwrap();

    let start = SystemTime::now();// - obj["data"]["time"];
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    let temp = &(obj["data"]["time"].as_f64());
    if temp.is_some() {
        //print_type_of(temp);
        println!("{}", since_the_epoch.as_millis() - (temp.unwrap() * 1000.0) as u128);
    }

}

fn print_on_msg(message_string: String) {
    println!("{}", message_string);
}

fn main() {
    let mut ftx_ws_client = FTXWSClient::new(Box::new(print_on_msg), None);
    let mut binance_ws_client = BinanceFuturesWSClient::new(Box::new(print_on_msg), None);

    let binance_channels = vec!["btcusdt@miniTicker".to_string()];
    let ftx_channels = vec!["ticker:BTC-PERP".to_string()];
    //channels_to_commands(channels, true);
    //    let new_channels = ws_client.channels_to_commands(channels, true);
    binance_ws_client.subscribe(&binance_channels);
    ftx_ws_client.subscribe(&ftx_channels);

    thread::spawn(|| {
        binance_ws_client.run(None);
    });

    thread::spawn(|| {
        ftx_ws_client.run(None);
    });
}