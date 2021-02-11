use crypto_ws_client::{FTXWSClient, WSClient};

#[macro_use]
mod utils;

#[test]
fn ftxperpetual() {
        gen_test!(
            FTXWSClient,
            &vec![
                "trades:BTC-PERP".to_string()
            ]
        );
}