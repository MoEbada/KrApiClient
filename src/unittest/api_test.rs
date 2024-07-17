#[cfg(test)]
mod tests {
    use httpmock::prelude::*;
    use serde_json::json;

    use crate::api::{check_connection, get_server_time, get_system_status, get_trading_pair};

    #[tokio::test]
    async fn test_get_server_time() {
        let server = MockServer::start();
        let server_time_json = json!({
            "error": [],
            "result": {
                "unixtime": 1_614_355_841,
                "rfc1123": "Mon, 15 Mar 2021 14:30:41 +0000"
            }
        });
        let server_time_mock = server.mock(|when, then| {
            when.method(GET).path("/Time");
            then.status(200).json_body(server_time_json);
        });

        let result = get_server_time(&server.url("")).await;
        server_time_mock.assert();
        assert!(result.is_ok());
        assert_eq!(result.unwrap().result.unwrap().unixtime, 1_614_355_841);
    }

    #[tokio::test]
    async fn test_get_system_status() {
        let server = MockServer::start();
        let system_status_json = json!({
          "error": [],
          "result": {
            "status": "online",
            "timestamp": "2023-04-01T12:00:00Z"
          }
        });
        let system_status_mock = server.mock(|when, then| {
            when.method(GET).path("/SystemStatus");
            then.status(200).json_body(system_status_json);
        });

        let result = get_system_status(&server.url("")).await;
        system_status_mock.assert();
        assert!(result.is_ok());
        assert_eq!(result.unwrap().result.unwrap().status, "online");
    }

    #[tokio::test]
    async fn test_check_connection() {
        let server = MockServer::start();
        let connection_mock = server.mock(|when, then| {
            when.method(GET);
            then.status(200);
        });

        let result = check_connection(&server.url("")).await;
        connection_mock.assert();
        assert!(result.is_ok());
        assert_eq!(result.unwrap().status, "Connection successful");
    }

    #[tokio::test]
    async fn test_get_xbtusd_pair() {
        let server = MockServer::start();
        let ticker_info_json = json!({
          "error": [],
          "result": {
            "XXBTZUSD": {
              "a": ["50000.00000", "1", "1.000"],
              "b": ["49995.00000", "2", "2.000"],
              "c": ["50000.00000", "0.500"],
              "v": ["1000.0000", "1500.0000"],
              "p": ["50000.00000", "49950.00000"],
              "t": [300, 500],
              "l": ["49000.00000", "48000.00000"],
              "h": ["51000.00000", "52000.00000"],
              "o": "49500.00000"
            }
          }
        });
        let xbtusd_pair_mock = server.mock(|when, then| {
            when.method(GET);
            then.status(200).json_body(ticker_info_json);
        });

        let result = get_trading_pair(&server.url(""), "").await;
        xbtusd_pair_mock.assert();
        assert!(result.is_ok());
        assert_eq!(
            result.unwrap().XXBTZUSD.get_ask(),
            ["50000.00000", "1", "1.000"]
        );
    }
}
