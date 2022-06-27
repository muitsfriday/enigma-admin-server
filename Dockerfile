FROM rust:latest

RUN mkdir -p /usr/src/app
WORKDIR /usr/src/app
COPY . /usr/src/app

RUN cargo install cargo-watch

EXPOSE 8080
