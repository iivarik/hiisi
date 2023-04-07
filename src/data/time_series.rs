use chrono::{DateTime, Duration, Utc};

#[derive(Debug)]
pub struct AssetTradeInfo {
    timestamp: DateTime<Utc>,
    open: f64,
    high: f64,
    low: f64,
    close: f64,
    volume: i64,
}

impl AssetTradeInfo {
    pub fn new(
        timestamp: DateTime<Utc>,
        open: f64,
        high: f64,
        low: f64,
        close: f64,
        volume: i64,
    ) -> AssetTradeInfo {
        AssetTradeInfo {
            timestamp,
            open,
            high,
            low,
            close,
            volume,
        }
    }
}

#[derive(Debug)]
pub struct TimeSeries {
    data: Vec<AssetTradeInfo>,
    interval: Duration,
}

impl TimeSeries {
    pub fn new(data: Vec<AssetTradeInfo>, interval: Duration) -> TimeSeries {
        TimeSeries { data, interval }
    }
}
