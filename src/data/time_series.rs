use chrono::{DateTime, Duration, Utc};

#[derive(Debug)]
pub struct PricePoint {
    timestamp: DateTime<Utc>,
    open: i64,
    high: i64,
    low: i64,
    close: i64,
    volume: i64,
}

#[derive(Debug)]
pub struct TimeSeries {
    data: Vec<PricePoint>,
    interval: Duration,
}

impl TimeSeries {
    pub fn new(data: Vec<PricePoint>, interval: Duration) -> TimeSeries {
        TimeSeries { data, interval }
    }
}
