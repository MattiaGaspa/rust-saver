FROM rust:1.79-slim-buster

RUN apt-get update --yes && apt-get full-upgrade --yes && apt install make -y
RUN rustup target add wasm32-unknown-unknown
RUN cargo install --locked trunk
RUN cargo install sqlx-cli --no-default-features --features rustls,postgres
RUN cargo install cargo-watch