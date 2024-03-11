# FROM ghcr.io/bmrlab/muse-credits:v1.1 as github-credits

# ENV start_params=" "

# EXPOSE 8080
# CMD ["sh", "-c", "/usr/app/credits-cli task ${task_params} && /usr/app/credits-cli start"]


FROM rust:1.74-slim as builder

WORKDIR /usr/src/

COPY . .

RUN apt-get install -y libssl-dev && apt-get -y install pkg-config

RUN cargo build --release

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y libc6 

WORKDIR /usr/app

COPY --from=builder /usr/src/config /usr/app/config
COPY --from=builder /usr/src/target/release/credits-cli /usr/app/credits-cli