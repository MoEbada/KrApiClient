# Kr API Client

Kr API Client is a command-line interface tool designed to interact with the Kr API. It provides various functionalities to query server time, system status, connection status, trading pair information, and access private API endpoints.

## Table of Contents

- [Kr API Client](#kr-api)
  - [Features](#features)
  - [Installation](#installation)
  - [Usage](#usage)
    - [Server Time](#server-time)
    - [System Status](#system-status)
    - [Connection Status](#connection-status)
    - [Trading Pair Information](#trading-pair-information)
    - [Private API Access](#private-api-access)
  - [Building from Source](#building-from-source)
  - [Docker](#docker)
  - [BDD Cucumber](#bdd-cucumber)

## Features

- **Server Time**: Fetches the current server time.
- **System Status**: Retrieves the current system status.
- **Connection Status**: Checks the connection status to the server.
- **Trading Pair Information**: Gets trading pair information for a given URL.
- **Private API Access**: Connects to a private API account to request specific data (e.g., Balance, OpenOrders).

## Installation

Before you can use Kr API CLient, ensure you have Rust and Cargo installed on your system. Follow the Rust installation guide here: [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install).

## Usage

To use Kr API CLient, you need to run it from the command line with the desired subcommand and its required arguments. Below are the available subcommands and their usage:

### Server Time
```sh
kr_api server-time --url <URL_BASE>
```

### System Status
```sh
kr_api system-status --url <URL_BASE>
```

### Connection Status
```sh
kr_api connection-status --url <URL_BASE>
```

### Trading Pair Information
```sh
kr_api trading-pair --url <URL_BASE> --pair_name <PAIR_NAME(e.g., XBTUSD)>
```

### Private API Access
```sh
kr_api private_api --url <URL base for private API> --requested_data <Data to request> --api_key <API Key> --private_api_key <Private API Key>
```

## Building from Source
To build Kr API Client from source, clone the repository and use Cargo to build the project:
```sh
git clone  https://github.com/MoEbada/KrApiClient.git
cd KrApiClient
cargo build --release
```
The executable will be located in target/release.

## Docker

To simplify the deployment and execution of Kr API Client, a Dockerfile is provided. This allows you to containerize the Kr API Client application, ensuring that it runs consistently across different environments. To use Docker for running Kr API Client, follow these steps:

1. **Build the Docker Image**: First, build the Docker image from the provided Dockerfile. This process involves setting up the Rust environment, installing necessary dependencies, and compiling the Kr API Client application. Use the following command to build the image:

    ```sh
    docker build -t kr_api:latest .
    ```

2. **Running the Container**: After the image is built, you can run Kr API Client within a Docker container. Replace `<command>` with the specific Kr API Client command you wish to execute (e.g., `connection-status`, `trading-pair`, or `private_api`). The `<URL>` and other parameters should be adjusted based on your needs:
 ```sh
    docker run kr_api <command> --url <URL>
 ```
or run it the docker image interactively using `-it` argument:

```sh
    docker run -it kr_api
```

This Docker setup ensures that you can easily deploy and run Kr API Client without worrying about the underlying system configurations or dependencies.


## BDD Cucumber

Gherkin tests are defined in `api.feature` and can be executed using cargo.
```sh
cargo test --test cucumber --features="output-junit"  
```