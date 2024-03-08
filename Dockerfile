FROM rust:1.74-slim as builder

WORKDIR /usr/src/

COPY . .

RUN cargo build --release

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y libc6

WORKDIR /usr/app

COPY --from=builder /usr/src/config /usr/app/config
COPY --from=builder /usr/src/target/release/credits-cli /usr/app/credits-cli

CMD ["sh", "-c", "cd /usr/app && ./credits-cli task trans_event_process url:'mongodb://root:sdffDDdffww!Dffss@dds-2ze73d97c4cb92941.mongodb.rds.aliyuncs.com:3717' && ./credits-cli start"]
