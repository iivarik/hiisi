use hiisi::alpha_vantage::api::Client;
use hiisi::data::TimeSeries;

fn main() {
    let api = Client::new("QYSZ8L88IKLHCTHP");
    let timeseries: TimeSeries = api
        .get_time_series_monthly_adjusted("AAPL", false)
        .unwrap()
        .into();

    println!("TimeSeries: {:?}", timeseries);
}
