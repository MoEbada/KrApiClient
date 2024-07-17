extern crate base64;
extern crate hmac;
extern crate sha2;

use hmac::{Hmac, Mac};
use serde::{Deserialize, Serialize};
use sha2::Digest;
use sha2::{Sha256, Sha512};
use std::collections::HashMap;
use std::error::Error as StdError;
use std::fmt;

#[derive(Debug)]
pub enum ApiError {
    Reqwest(reqwest::Error),
    NotFound(String),
}

impl PairData {
    pub fn get_ask(&self) -> Vec<String> {
        return self.a.clone();
    }
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ApiError::Reqwest(ref err) => write!(f, "Request error: {}", err),
            ApiError::NotFound(ref msg) => write!(f, "Not found: {}", msg),
        }
    }
}

impl StdError for ApiError {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match *self {
            ApiError::Reqwest(ref err) => Some(err),
            ApiError::NotFound(_) => None,
        }
    }
}

impl From<reqwest::Error> for ApiError {
    fn from(err: reqwest::Error) -> ApiError {
        ApiError::Reqwest(err)
    }
}

#[derive(Deserialize, Debug, Serialize)]
pub struct ServerTime {
    pub error: Vec<String>,
    pub result: Option<TimeResult>,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct TimeResult {
    pub unixtime: u64,
    pub rfc1123: String,
}

#[derive(Deserialize, Debug)]
pub struct SystemStatus {
    pub error: Vec<String>,
    pub result: Option<StatusResult>,
}

#[derive(Deserialize, Debug)]
pub struct StatusResult {
    pub status: String,
    pub timestamp: String,
}

#[derive(Deserialize, Debug)]
pub struct ConnectionStatus {
    pub status: String,
    pub timestamp: String,
}

#[derive(Deserialize, Debug)]
pub struct TickerResponse {
    pub error: Vec<String>,
    pub result: Option<TickerInfo>,
}

#[derive(Deserialize, Debug)]
pub struct TickerInfo {
    pub XXBTZUSD: PairData,
}

#[derive(Deserialize, Debug)]
pub struct PairData {
    a: Vec<String>, // Ask array(<price>, <whole lot volume>, <lot volume>),
    b: Vec<String>,
    c: Vec<String>,
    v: Vec<String>,
    p: Vec<String>,
    t: Vec<u64>,
    l: Vec<String>,
    h: Vec<String>,
    o: String,
}

pub fn get_kr_signature(
    urlpath: &str,
    data: &HashMap<&str, String>,
    secret: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let postdata = serde_urlencoded::to_string(data)?;
    let encoded = format!("{}{}", data["nonce"], postdata).into_bytes();
    let message = [urlpath.as_bytes(), &Sha256::digest(&encoded)[..]].concat();

    let mut mac = Hmac::<Sha512>::new_from_slice(&base64::decode(secret)?.as_slice())?;
    mac.update(&message);
    let sigdigest = mac.finalize().into_bytes();
    Ok(base64::encode(sigdigest))
}
