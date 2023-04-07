use chrono::{DateTime, Duration, NaiveDate, NaiveDateTime, NaiveTime, TimeZone, Utc};
use reqwest;
use serde::{de, Deserialize, Deserializer};
use std::collections::HashMap;
use std::fmt::Display;
use std::str::FromStr;
use url::Url;

use super::error::AlphaVantageError;
use crate::data::{AssetTradeInfo, TimeSeries};

const API_BASE_URL: &str = "https://alphavantage.co/query";
const TIME_SERIES_DAILY: &str = "TIME_SERIES_DAILY";
const TIME_SERIES_DAILY_ADJUSTED: &str = "TIME_SERIES_DAILY_ADJUSTED";
const TIME_SERIES_WEEKLY: &str = "TIME_SERIES_WEEKLY";
const TIME_SERIES_WEEKLY_ADJUSTED: &str = "TIME_SERIES_WEEKLY_ADJUSTED";
const TIME_SERIES_MONTHLY: &str = "TIME_SERIES_MONTHLY";
const TIME_SERIES_MONTHLY_ADJUSTED: &str = "TIME_SERIES_MONTHLY_ADJUSTED";

#[derive(Deserialize, Debug)]
struct MetaData {
    #[serde(rename = "1. Information")]
    information: String,
    #[serde(rename = "2. Symbol")]
    symbol: String,
    #[serde(rename = "3. Last Refreshed")]
    last_refreshed: String,
    #[serde(rename = "4. Time Zone")]
    tz: String,
}

#[derive(Deserialize, Debug)]
struct MetaDataWithOutputSize {
    #[serde(rename = "1. Information")]
    information: String,
    #[serde(rename = "2. Symbol")]
    symbol: String,
    #[serde(rename = "3. Last Refreshed")]
    last_refreshed: String,
    #[serde(rename = "4. Output Size")]
    size: String,
    #[serde(rename = "5. Time Zone")]
    tz: String,
}

#[derive(Deserialize, Debug)]
struct PriceInfo {
    #[serde(rename = "1. open", deserialize_with = "deserialize_from_str")]
    open: f64,
    #[serde(rename = "2. high", deserialize_with = "deserialize_from_str")]
    high: f64,
    #[serde(rename = "3. low", deserialize_with = "deserialize_from_str")]
    low: f64,
    #[serde(rename = "4. close", deserialize_with = "deserialize_from_str")]
    close: f64,
    #[serde(rename = "5. volume", deserialize_with = "deserialize_from_str")]
    volume: i64,
}

#[derive(Deserialize, Debug)]
struct AdjustedPriceInfo {
    #[serde(rename = "1. open", deserialize_with = "deserialize_from_str")]
    open: f64,
    #[serde(rename = "2. high", deserialize_with = "deserialize_from_str")]
    high: f64,
    #[serde(rename = "3. low", deserialize_with = "deserialize_from_str")]
    low: f64,
    #[serde(rename = "4. close", deserialize_with = "deserialize_from_str")]
    close: f64,
    #[serde(
        rename = "5. adjusted close",
        deserialize_with = "deserialize_from_str"
    )]
    adjusted_close: f64,
    #[serde(rename = "6. volume", deserialize_with = "deserialize_from_str")]
    volume: i64,
    #[serde(
        rename = "7. dividend amount",
        deserialize_with = "deserialize_from_str"
    )]
    dividend: f64,
    #[serde(
        rename = "8. split coefficient",
        deserialize_with = "deserialize_from_str",
        default
    )]
    split_coefficient: f64,
}

#[derive(Deserialize, Debug)]
pub struct TimeSeriesDailyResponse {
    #[serde(rename = "Meta Data")]
    metadata: MetaDataWithOutputSize,
    #[serde(rename = "Time Series (Daily)")]
    prices: HashMap<NaiveDate, PriceInfo>,
}

#[derive(Deserialize, Debug)]
pub struct TimeSeriesDailyAdjustedResponse {
    #[serde(rename = "Meta Data")]
    metadata: MetaDataWithOutputSize,
    #[serde(rename = "Time Series (Daily)")]
    prices: HashMap<NaiveDate, AdjustedPriceInfo>,
}

#[derive(Deserialize, Debug)]
pub struct TimeSeriesWeeklyResponse {
    #[serde(rename = "Meta Data")]
    metadata: MetaData,
    #[serde(rename = "Weekly Time Series")]
    prices: HashMap<NaiveDate, PriceInfo>,
}

#[derive(Deserialize, Debug)]
pub struct TimeSeriesWeeklyAdjustedResponse {
    #[serde(rename = "Meta Data")]
    metadata: MetaData,
    #[serde(rename = "Weekly Adjusted Time Series")]
    prices: HashMap<NaiveDate, AdjustedPriceInfo>,
}

#[derive(Deserialize, Debug)]
pub struct TimeSeriesMonthlyResponse {
    #[serde(rename = "Meta Data")]
    metadata: MetaData,
    #[serde(rename = "Monthly Time Series")]
    prices: HashMap<NaiveDate, PriceInfo>,
}

#[derive(Deserialize, Debug)]
pub struct TimeSeriesMonthlyAdjustedResponse {
    #[serde(rename = "Meta Data")]
    metadata: MetaData,
    #[serde(rename = "Monthly Adjusted Time Series")]
    prices: HashMap<NaiveDate, AdjustedPriceInfo>,
}

impl Into<TimeSeries> for TimeSeriesDailyResponse {
    fn into(self) -> TimeSeries {
        let asset_info = self
            .prices
            .into_iter()
            .map(|(date, info)| {
                AssetTradeInfo::new(
                    naive_date_to_datetime_utc(date),
                    info.open,
                    info.high,
                    info.low,
                    info.close,
                    info.volume,
                )
            })
            .collect();
        TimeSeries::new(asset_info, Duration::days(1))
    }
}

impl Into<TimeSeries> for TimeSeriesMonthlyAdjustedResponse {
    fn into(self) -> TimeSeries {
        let asset_info = self
            .prices
            .into_iter()
            .map(|(date, info)| {
                AssetTradeInfo::new(
                    naive_date_to_datetime_utc(date),
                    info.open,
                    info.high,
                    info.low,
                    info.close,
                    info.volume,
                )
            })
            .collect();
        TimeSeries::new(asset_info, Duration::days(1))
    }
}

fn naive_date_to_datetime_utc(date: NaiveDate) -> DateTime<Utc> {
    let time = NaiveTime::from_hms_opt(0, 0, 0).unwrap();
    let dt = NaiveDateTime::new(date, time);

    Utc.from_utc_datetime(&dt)
}

fn deserialize_from_str<'de, S, D>(deserializer: D) -> Result<S, D::Error>
where
    S: FromStr,
    S::Err: Display,
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    S::from_str(&s).map_err(de::Error::custom)
}

pub struct Client {
    api_key: String,
    api_url: String,
}

impl Client {
    pub fn new(api_key: &str) -> Client {
        Client {
            api_key: api_key.to_owned(),
            api_url: API_BASE_URL.to_owned(),
        }
    }

    pub fn new_with_url(api_key: &str, api_url: &str) -> Client {
        Client {
            api_key: api_key.to_owned(),
            api_url: api_url.to_owned(),
        }
    }

    pub fn get_time_series_daily(
        &self,
        symbol: &str,
        full_history: bool,
    ) -> Result<TimeSeriesDailyResponse, AlphaVantageError> {
        let url = self.make_url(TIME_SERIES_DAILY, symbol, full_history)?;
        let response = reqwest::blocking::get(url)?.json::<TimeSeriesDailyResponse>()?;

        Ok(response)
    }

    pub fn get_time_series_daily_adjusted(
        &self,
        symbol: &str,
        full_history: bool,
    ) -> Result<TimeSeriesDailyAdjustedResponse, AlphaVantageError> {
        let url = self.make_url(TIME_SERIES_DAILY_ADJUSTED, symbol, full_history)?;
        let response = reqwest::blocking::get(url)?.json::<TimeSeriesDailyAdjustedResponse>()?;

        Ok(response)
    }

    pub fn get_time_series_weekly(
        &self,
        symbol: &str,
        full_history: bool,
    ) -> Result<TimeSeriesWeeklyResponse, AlphaVantageError> {
        let url = self.make_url(TIME_SERIES_WEEKLY, symbol, full_history)?;
        let response = reqwest::blocking::get(url)?.json::<TimeSeriesWeeklyResponse>()?;

        Ok(response)
    }

    pub fn get_time_series_weekly_adjusted(
        &self,
        symbol: &str,
        full_history: bool,
    ) -> Result<TimeSeriesWeeklyAdjustedResponse, AlphaVantageError> {
        let url = self.make_url(TIME_SERIES_WEEKLY_ADJUSTED, symbol, full_history)?;
        let response = reqwest::blocking::get(url)?.json::<TimeSeriesWeeklyAdjustedResponse>()?;

        Ok(response)
    }

    pub fn get_time_series_monthly(
        &self,
        symbol: &str,
        full_history: bool,
    ) -> Result<TimeSeriesMonthlyResponse, AlphaVantageError> {
        let url = self.make_url(TIME_SERIES_MONTHLY, symbol, full_history)?;
        let response = reqwest::blocking::get(url)?.json::<TimeSeriesMonthlyResponse>()?;

        Ok(response)
    }

    pub fn get_time_series_monthly_adjusted(
        &self,
        symbol: &str,
        full_history: bool,
    ) -> Result<TimeSeriesMonthlyAdjustedResponse, AlphaVantageError> {
        let url = self.make_url(TIME_SERIES_MONTHLY_ADJUSTED, symbol, full_history)?;
        let response = reqwest::blocking::get(url)?.json::<TimeSeriesMonthlyAdjustedResponse>()?;

        Ok(response)
    }

    fn make_url(
        &self,
        function: &str,
        symbol: &str,
        extended: bool,
    ) -> Result<Url, AlphaVantageError> {
        let mut url = Url::parse(self.api_url.as_str())?;
        url.set_path("query");
        url.query_pairs_mut()
            .append_pair("function", function)
            .append_pair("symbol", symbol)
            .append_pair("apikey", &self.api_key);

        if extended {
            url.query_pairs_mut().append_pair("outputsize", "full");
        }

        Ok(url)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize_price_info() {
        let data = r#"{
            "1. open": "10.0",
            "2. high": "20.0",
            "3. low": "5.0",
            "4. close": "15.0",
            "5. volume": "1000"
        }"#;

        let price_info: PriceInfo = serde_json::from_str(data).unwrap();

        assert_eq!(price_info.open, 10.0);
        assert_eq!(price_info.high, 20.0);
        assert_eq!(price_info.low, 5.0);
        assert_eq!(price_info.close, 15.0);
        assert_eq!(price_info.volume, 1000);
    }

    #[test]
    fn test_deserialize_adjusted_price_info() {
        let data = r#"{
            "1. open": "10.0",
            "2. high": "20.0",
            "3. low": "5.0",
            "4. close": "15.0",
            "5. adjusted close": "14.0",
            "6. volume": "1000",
            "7. dividend amount": "0.1",
            "8. split coefficient": "1.0"
        }"#;

        let adjusted_price_info: AdjustedPriceInfo = serde_json::from_str(data).unwrap();

        assert_eq!(adjusted_price_info.open, 10.0);
        assert_eq!(adjusted_price_info.high, 20.0);
        assert_eq!(adjusted_price_info.low, 5.0);
        assert_eq!(adjusted_price_info.close, 15.0);
        assert_eq!(adjusted_price_info.adjusted_close, 14.0);
        assert_eq!(adjusted_price_info.volume, 1000);
        assert_eq!(adjusted_price_info.dividend, 0.1);
        assert_eq!(adjusted_price_info.split_coefficient, 1.0);
    }

    #[test]
    fn test_client_make_url() {
        let client = Client::new("test_api_key");
        let url = client.make_url(TIME_SERIES_DAILY, "MSFT", true).unwrap();

        assert_eq!(
            url.to_string(),
            "https://alphavantage.co/query?function=TIME_SERIES_DAILY&symbol=MSFT&apikey=test_api_key&outputsize=full"
        );
    }
}
