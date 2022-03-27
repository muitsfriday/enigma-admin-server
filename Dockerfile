FROM rust:latest

RUN mkdir -p /usr/src/enigma-server
WORKDIR /usr/src/enigma-server
COPY . /usr/src/enigma-server
# COPY init-user-db.sh /docker-entrypoint-initdb.d/

RUN rustc --version
RUN cargo install --path ./
RUN cargo install cargo-watch
RUN cargo install diesel_cli

EXPOSE 8080
