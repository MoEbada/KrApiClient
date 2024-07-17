use clap::{App, SubCommand};
use kr_api::api::{check_connection, get_server_time, get_system_status, get_trading_pair};
use kr_api::private_api::get_private_api_data;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let matches = App::new("Kr CLI")
        .version("0.1.0")
        .about("Interacts with Kr API")
        .subcommand(SubCommand::with_name("connection-status")
            .arg(clap::Arg::with_name("url")
                .required(true)
                .help("The URL to check the connection status for")).about("Gets connection status to server"))
        .subcommand(SubCommand::with_name("server-time")
            .arg(clap::Arg::with_name("url")
                .required(true)
                .help("The URL to server API")).about("Gets server time"))
        .subcommand(SubCommand::with_name("system-status")
            .arg(clap::Arg::with_name("url")
                .required(true)
                .help("The URL to server API")).about("Gets system status"))
        .subcommand(SubCommand::with_name("trading-pair")
            .arg(clap::Arg::with_name("url")
                .required(true)
                .help("The base URL to server API"))
            .arg(clap::Arg::with_name("pair_name")
                .required(true)
                .help("Trading pair name to be retrieved (e.g., XBTUSD (case-sensitive)")).about("Gets trading pair info for given url"))
        .subcommand(SubCommand::with_name("private_api")
            .arg(clap::Arg::with_name("url")
                .required(true)
                .help("URL base for private API"))
            .arg(clap::Arg::with_name("requested_data")
                .required(true)
                .help("Data to request on private API (e.g. OpenOrders, Balance..etc (Case-sensitive))"))
            .arg(clap::Arg::with_name("api_key")
                .required(true)
                .help("API Key"))
            .arg(clap::Arg::with_name("private_api_key")
                .required(true)
                .help("Private API Key to access private API"))
            .about("Connects to private API account"))   
        .get_matches();

    if let Some(_) = matches.subcommand_matches("server-time") {
        let subcommand_matches = matches.subcommand_matches("server-time").unwrap();
        let url = subcommand_matches.value_of("url").unwrap();
        let result = get_server_time(url).await;
        match result {
            Ok(server_time) => println!("{:?}", server_time),
            Err(e) => {
                eprintln!("Error: {}", e);
                return Err(e);
            }
        }
    } else if let Some(_) = matches.subcommand_matches("system-status") {
        let subcommand_matches = matches.subcommand_matches("system-status").unwrap();
        let url = subcommand_matches.value_of("url").unwrap();
        let result = get_system_status(url).await;
        match result {
            Ok(system_status) => println!("{:?}", system_status),
            Err(e) => {
                eprintln!("Error: {}", e);
                return Err(e);
            }
        }
    } else if let Some(_) = matches.subcommand_matches("connection-status") {
        let subcommand_matches = matches.subcommand_matches("connection-status").unwrap();
        let url = subcommand_matches.value_of("url").unwrap();
        let result = check_connection(url).await;
        match result {
            Ok(connection_status) => println!("{:?}", connection_status),
            Err(e) => {
                eprintln!("Error: {}", e);
                return Err(e);
            }
        }
    } else if let Some(_) = matches.subcommand_matches("trading-pair") {
        let subcommand_matches = matches.subcommand_matches("trading-pair").unwrap();
        let url = subcommand_matches.value_of("url").unwrap();
        let trading_pair = subcommand_matches.value_of("pair_name").unwrap();
        let result = get_trading_pair(url, trading_pair).await;
        match result {
            Ok(system_status) => println!("{:?}", system_status),
            Err(e) => eprintln!("Error: {}", e),
        }
    } else if let Some(_) = matches.subcommand_matches("private_api") {
        let subcommand_matches = matches.subcommand_matches("private_api").unwrap();
        let base_url = subcommand_matches.value_of("url").unwrap();
        let requested_data = subcommand_matches.value_of("requested_data").unwrap();
        let private_api_key = subcommand_matches.value_of("private_api_key").unwrap();
        let api_key = subcommand_matches.value_of("api_key").unwrap();
        let result = get_private_api_data(base_url, requested_data, api_key, private_api_key).await;
        match result {
            Ok(response) => println!("{:?}", response),
            Err(e) => eprintln!("Error: {}", e),
        }
    } else {
        println!("No valid subcommand provided. Supported subcommands: server-time, system-status, connection-status, connection-status, private_api");
    }
    Ok(())
}
