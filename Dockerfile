FROM rust:1.76.0-buster

WORKDIR /usr/src/shiprs

COPY . .

RUN cargo build --release
