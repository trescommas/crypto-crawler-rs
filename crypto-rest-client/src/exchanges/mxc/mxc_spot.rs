use super::super::utils::http_get;
use crate::error::Result;
use std::collections::HashMap;

const BASE_URL: &str = "https://www.mxc.co";

/// MXC Spot market.
///
/// * REST API doc: <https://mxcdevelop.github.io/APIDoc/>
/// * Trading at: <https://www.mxc.com/trade/pro>
pub struct MxcSpotRestClient {
    _access_key: String,
    _secret_key: Option<String>,
}

impl MxcSpotRestClient {
    pub fn new(access_key: String, secret_key: Option<String>) -> Self {
        MxcSpotRestClient {
            _access_key: access_key,
            _secret_key: secret_key,
        }
    }

    /// Get latest trades.
    ///
    /// 1000 trades are returned.
    ///
    /// For example: <https://www.mxc.co/open/api/v2/market/deals?symbol=BTC_USDT&limit=1000&api_key=your-access-key>
    #[allow(non_snake_case)]
    pub fn fetch_trades(symbol: &str) -> Result<String> {
        if std::env::var("MXC_ACCESS_KEY").is_err() {
            panic!("MXC Spot REST APIs require access key, please set it to the MXC_ACCESS_KEY environment variable");
        }
        let access_key = std::env::var("MXC_ACCESS_KEY").unwrap();

        gen_api!(format!(
            "/open/api/v2/market/deals?symbol={}&limit=1000&api_key={}",
            symbol, access_key
        ))
    }

    /// Get latest Level2 snapshot of orderbook.
    ///
    /// Top 2000 bids and asks will be returned.
    ///
    /// For example: <https://www.mxc.co/open/api/v2/market/depth?symbol=BTC_USDT&depth=2000&api_key=your-access-key>
    pub fn fetch_l2_snapshot(symbol: &str) -> Result<String> {
        if std::env::var("MXC_ACCESS_KEY").is_err() {
            panic!("MXC Spot REST APIs require access key, please set it to the MXC_ACCESS_KEY environment variable");
        }
        let access_key = std::env::var("MXC_ACCESS_KEY").unwrap();

        gen_api!(format!(
            "/open/api/v2/market/depth?symbol={}&depth=2000&api_key={}",
            symbol, access_key
        ))
    }
}
