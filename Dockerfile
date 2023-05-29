
FROM rust:1.69 as build

RUN USER=root cargo new --bin tjan_mcsr
WORKDIR /tjan_mcsr

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

RUN cargo build --release
RUN rm src/*.rs

COPY ./src ./src

RUN rm ./target/release/deps/tjan_mcsr*
RUN cargo build --release



FROM rust:1.69-slim-buster

COPY --from=build /tjan_mcsr/target/release/tjan_mcsr .
COPY config.json .

CMD ["./tjan_mcsr"]
