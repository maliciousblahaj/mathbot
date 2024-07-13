FROM lukemathwalker/cargo-chef:latest-rust-1 AS chef

WORKDIR /mathbot

FROM chef AS planner
COPY Cargo.toml ./
COPY Cargo.lock ./
COPY ./src ./src
COPY ./migrations ./migrations
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /mathbot/recipe.json recipe.json
#build and cache dependencies
RUN cargo chef cook --release --recipe-path recipe.json
#build application
COPY Cargo.toml ./
COPY Cargo.lock ./
COPY ./src ./src
COPY ./migrations ./migrations
RUN cargo build --release --bin mathbot

FROM debian:bookworm-slim AS runtime
WORKDIR /mathbot
COPY --from=builder /mathbot/target/release/mathbot /usr/local/bin
ENTRYPOINT [ "/usr/local/bin/mathbot" ]

#requires a volume in docker compose
#  volumes:
#    - ./db:/mathbot/db