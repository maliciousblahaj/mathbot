FROM rust:latest

WORKDIR /mathbot

COPY Cargo.toml ./
COPY Cargo.lock ./

COPY ./src ./src
COPY ./migrations ./migrations

RUN cargo build --release

CMD ["./target/release/mathbot > ./log.txt"]

#requires a volume in docker compose
#  volumes:
#    - ./db:/mathbot/db
