FROM rust:alpine

WORKDIR /app

RUN apk update && apk add pkgconfig openssl-dev musl-dev

COPY Cargo.toml .
COPY dummy.rs .

# Cache building
RUN sed -i 's|src/main.rs|dummy.rs|' Cargo.toml
RUN cargo build --release
RUN cargo build
RUN sed -i 's|dummy.rs|src/main.rs|' Cargo.toml

# Building normally
COPY src src
RUN cargo build
RUN cargo install --path .

ENTRYPOINT [ "phuncache" ]
