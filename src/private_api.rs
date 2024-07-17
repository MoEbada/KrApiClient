extern crate chrono;
extern crate reqwest;
extern crate serde;

use crate::utils::get_kr_signature;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct BalanceResponse {
    pub error: Vec<String>,
    pub result: HashMap<String, String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OpenOrdersResponse {
    pub error: Vec<String>,
    pub result: OpenOrdersResult,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OpenOrdersResult {
    pub open: HashMap<String, OrderInfo>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OrderInfo {
    pub refid: Option<String>,
    pub userref: Option<i32>,
    pub status: String,
    pub opentm: f64,
    pub starttm: f64,
    pub expiretm: f64,
    pub descr: OrderDescription,
    pub vol: String,
    pub vol_exec: String,
    pub cost: String,
    pub fee: String,
    pub price: String,
    pub stopprice: String,
    pub limitprice: String,
    pub misc: String,
    pub oflags: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OrderDescription {
    pub pair: String,
    pub type_: String,
    pub ordertype: String,
    pub price: String,
    pub price2: String,
    pub leverage: String,
    pub order: String,
    pub close: String,
}
#[derive(Debug)]
pub enum ResponseType {
    Balance(BalanceResponse),
    OpenOrders(OpenOrdersResponse),
    // Add more response types for different private API data requests here
}

pub async fn get_private_api_data(
    base_url: &str,
    requested_data: &str,
    api_key: &str,
    private_api_key: &str,
) -> Result<ResponseType, Box<dyn std::error::Error>> {
    let uri_path = format!("/0/private/{}", requested_data);
    let api_url = format!("{}{}", base_url, uri_path);

    println!("API URL: {}", api_url);
    // Generate nonce
    let nonce = chrono::Utc::now().timestamp_millis().to_string();

    // Prepare POST data
    let mut post_data = HashMap::new();
    post_data.insert("nonce", nonce.clone());

    // Calculate signature
    let api_signature = get_kr_signature(&uri_path, &post_data, private_api_key).unwrap();

    // Prepare headers
    let client = reqwest::Client::new();
    let response = client
        .post(&api_url)
        .header("API-Key", api_key)
        .header("API-Sign", api_signature)
        .form(&post_data)
        .send()
        .await?;

    match requested_data {
        "Balance" => {
            let response = response.json::<BalanceResponse>().await?;
            Ok(ResponseType::Balance(response))
        }
        "OpenOrders" => {
            let response = response.json::<OpenOrdersResponse>().await?;
            println!("Response: {:?}", response);
            Ok(ResponseType::OpenOrders(response))
        }
        _ => {
            panic!("currently 'Balance' and 'OpenOrders' are the only supported private API data requests");
        }
    }
}
