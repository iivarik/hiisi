use hiisi::alpha_vantage::api::Client;

fn main() {
    let api = Client::new("QYSZ8L88IKLHCTHP");
    let timeseries = api.get_time_series_monthly_adjusted("AAPL", false).unwrap();

    for (date, price_info) in timeseries.prices.iter() {
        println!("Date: {:?} Volume: {:?}", date, price_info.volume);
    }
}
