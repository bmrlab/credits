FROM rust:1.74-slim as builder

WORKDIR /usr/src/

COPY . .

RUN cargo build --release

FROM debian:bookworm-slim

WORKDIR /usr/app

COPY --from=builder /usr/src/config /usr/app/config
COPY --from=builder /usr/src/target/release/muse_integrator-cli /usr/app/muse_integrator-cli

ENTRYPOINT ["/usr/app/muse_integrator-cli"]