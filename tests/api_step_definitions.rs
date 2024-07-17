use cucumber::{given, then, when, writer, World};
use kr_api::{
    api::{check_connection, get_server_time, get_system_status, get_trading_pair},
    private_api::{get_private_api_data, ResponseType},
};
use std::time::{SystemTime, UNIX_EPOCH};
use std::{collections::HashMap, fs::File, io::BufWriter};

const BASE_URL: &str = "https://api.example.com"; // Replace with the actual API URL when running the tests

#[derive(cucumber::World, Debug, Default)]
pub struct ApiWorld {
    url: String,
    server_status: String,
    server_time: u64,
    pair_info_ask_array: Vec<String>,
    balance: HashMap<String, String>,
}

#[given(regex = r"^Connection to (?P<public_or_private>.+) server API is succesful$")]
async fn connection_successful(world: &mut ApiWorld, public_or_private: String) {
    println!("Provide a valide API  URL for the tests to run"); // Remove this line after updating the BASE_URL
    if public_or_private == "public" {
        world.url = format!("{}{}", BASE_URL, "/0/public");
    } else if public_or_private == "private" {
        world.url = format!("{}{}", BASE_URL, "/0/private");
    } else {
        panic!("Invalid API type");
    }
    assert!(check_connection(&world.url).await.is_ok());
    assert!(check_connection(&world.url).await.unwrap().status == "Connection successful");
}

#[given("Connection to public server API for ticker is succesful")]
async fn connection_to_pair_api_successful(world: &mut ApiWorld) {
    world.url = format!("{}{}", BASE_URL, "/0/public");
    let url = format!("{}{}", world.url, "/Ticker");
    assert!(check_connection(&url).await.is_ok());
    assert!(check_connection(&url).await.unwrap().status == "Connection successful");
}

#[when("I request the system status")]
async fn request_system_status(world: &mut ApiWorld) {
    let server_status = get_system_status(&world.url).await;
    world.server_status = server_status.unwrap().result.unwrap().status;
}

#[when("I request the server time")]
async fn request_server_time(world: &mut ApiWorld) {
    let server_time = get_server_time(&world.url).await;
    world.server_time = server_time.unwrap().result.unwrap().unixtime;
}

#[when(regex = r"^I request the (?P<pair_name>.+) pair$")]
async fn request_trading_pair(world: &mut ApiWorld, pair_name: String) {
    let pair_info = get_trading_pair(&world.url, &pair_name).await;
    println!("{:?}", pair_info);
    world.pair_info_ask_array = pair_info.unwrap().XXBTZUSD.get_ask();
}

#[when(
    regex = r"^I request the (?P<requested_data>.+) for (?P<api_key>.+) and (?P<private_api_key>.+)$"
)]
async fn request_uri_on_private_api(
    world: &mut ApiWorld,
    requested_data: String,
    api_key: String,
    private_api_key: String,
) {
    world.url = BASE_URL.to_string();
    let response =
        get_private_api_data(&world.url, &requested_data, &api_key, &private_api_key).await;
    println!("{:?}", response);
    match response.unwrap() {
        ResponseType::Balance(balance_response) => world.balance = balance_response.result,
        _ => panic!("Not a BalanceResponse variant"),
    };
}

#[then(regex = r"^System status should be (?P<expected_status>.+)$")]
async fn system_status_should_be_online(world: &mut ApiWorld, expected_status: String) {
    assert!(world.server_status == expected_status);
}

#[then(regex = r"^Server time difference from system time should be less than (\d) seconds$")]
async fn server_time_should_be_today(world: &mut ApiWorld, difference_in_seconds: u64) {
    // Get the current time as a timestamp in seconds
    let current_timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs();
    // Calculate the difference in seconds
    let difference_in_secs = current_timestamp
        .checked_sub(world.server_time)
        .expect("Provided timestamp is in the future");
    assert!(difference_in_secs < difference_in_seconds);
}

#[then("Pair info should not be empty")]
async fn verify_pair_info_not_empty(world: &mut ApiWorld) {
    assert!(!world.pair_info_ask_array.is_empty());
}

#[then(regex = r"^Cash balance should be greater than (\d)$")]
async fn verify_cash_balance(world: &mut ApiWorld, expected_balance_threshold: f64) {
    check_balance("ZEUR", expected_balance_threshold, world.balance.clone());
}

#[then(regex = r"^Asset balance should be greater than (\d)$")]
async fn verify_asset_balance(world: &mut ApiWorld, expected_ethereum_threshold: f64) {
    check_balance("XETH", expected_ethereum_threshold, world.balance.clone());
}

fn check_balance(balance_type: &str, threshold: f64, retrieved_balance: HashMap<String, String>) {
    let actual_balance = retrieved_balance
        .get(balance_type)
        .unwrap()
        .parse::<f64>()
        .unwrap();
    println!("expected_balance_threshold: {:?}", threshold);
    println!("actual_balance: {:?}", actual_balance);
    assert!(actual_balance > threshold);
}

#[tokio::main]
async fn main() {
    let output_file = File::create("junit.xml").expect("Failed to create output file");
    let writer = BufWriter::new(output_file);

    ApiWorld::cucumber()
        .with_writer(writer::JUnit::new(writer, 1))
        .run("C:/Users/moham/workspace/KrApiKey/features/api.feature")
        .await;
    ApiWorld::run("C:/Users/moham/workspace/KrApiKey/features/api.feature").await;
}
