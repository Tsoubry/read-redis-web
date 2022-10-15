FROM rust:1.64.0-buster as build

WORKDIR /app/

COPY src/ /app/src/
COPY Cargo.toml /app/Cargo.toml
COPY Cargo.lock /app/Cargo.lock

RUN cargo build --release


FROM debian:stable-slim

WORKDIR /app/

COPY --from=build /app/target/release/read-redis-web .

EXPOSE 8080

ENTRYPOINT ["/app/read-redis-web"]
