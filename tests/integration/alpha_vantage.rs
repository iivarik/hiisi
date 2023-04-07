use hiisi::alpha_vantage::api::Client;
use httpmock::{prelude::*, Mock};

fn mock_time_series_daily_adjusted_response(server: &MockServer) -> Mock {
    let body = include_str!("fixtures/time_series_daily_adjusted_response.json");

    server.mock(|when, then| {
        when.method(GET)
            .path("/query")
            .query_param("function", "TIME_SERIES_DAILY_ADJUSTED")
            .query_param("symbol", "IBM")
            .query_param("apikey", "test_api_key")
            .query_param("outputsize", "full");
        then.status(200)
            .header("content-type", "application/json")
            .body(body);
    })
}

#[test]
fn test_get_time_series_daily_adjusted() {
    let _ = dotenv::dotenv().ok();
    let server = MockServer::start();

    let api_key = "test_api_key";
    let client = Client::new_with_url(api_key, server.base_url().as_str());
    let mock = mock_time_series_daily_adjusted_response(&server);

    let response = client
        .get_time_series_daily_adjusted("IBM", true)
        .expect("Failed to get time series daily data");

    // assert_eq!(response.metadata.symbol, "IBM");

    mock.assert();
}
