FROM rust:alpine

WORKDIR /app

COPY Cargo.toml /app/Cargo.toml
COPY Cargo.lock  /app/Cargo.lock
COPY src /app/src
RUN apk update && apk add pkgconfig openssl-dev musl-dev

RUN cargo build --release
RUN cargo install --path .

ENTRYPOINT [ "phuncache" ]