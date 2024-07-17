use crate::utils::*;
use reqwest::Error;

pub async fn get_server_time(url: &str) -> Result<ServerTime, Error> {
    let url = format!("{}/Time", url);
    let response = reqwest::get(&url).await?.json::<ServerTime>().await?;
    Ok(response)
}

pub async fn get_system_status(url: &str) -> Result<SystemStatus, Error> {
    let url = format!("{}/SystemStatus", url);
    let response = reqwest::get(&url).await?.json::<SystemStatus>().await?;
    Ok(response)
}

pub async fn check_connection(url: &str) -> Result<ConnectionStatus, Error> {
    match reqwest::get(url).await {
        Ok(_) => Ok(ConnectionStatus {
            status: "Connection successful".to_string(),
            timestamp: chrono::Utc::now().to_rfc2822(),
        }),
        Err(e) => Err(e),
    }
}

pub async fn get_trading_pair(url: &str, pair: &str) -> Result<TickerInfo, ApiError> {
    let trading_pair_url = format!("{}/Ticker?pair={}", url, pair);
    println!("URL: {}", trading_pair_url);
    let response = reqwest::get(trading_pair_url)
        .await?
        .json::<TickerResponse>()
        .await?;

    match response.result {
        Some(ticker_info) => Ok(ticker_info),
        None => Err(ApiError::NotFound("Pair not found".to_string())),
    }
}
