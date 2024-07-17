# 1. This tells docker to use the Rust official image
FROM rust:slim-bullseye

# 2. Install pkg-config, OpenSSL development packages, and Python
RUN apt-get update && apt-get install -y pkg-config libssl-dev python3 python3-pip

# 3. Copy the files in your machine to the Docker image
COPY ./ ./

# 4. Build your program for release
RUN cargo build

# 5. Start a bash shell by default
CMD ["bash"]