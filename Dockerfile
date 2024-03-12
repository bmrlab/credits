# FROM dockerhub.tezign.com/innovation/muse-credits/muse-credits:v1.6

# ENV start_params=" "

# EXPOSE 8080
# CMD ["sh", "-c", "/usr/app/credits-cli task ${task_params} && /usr/app/credits-cli start"]



FROM rust:1.74-slim as builder


WORKDIR /usr/src/

RUN mkdir ~/.cargo/ && touch ~/.cargo/config \
    && echo '[source.crates-io]' > ~/.cargo/config \
    && echo "replace-with = 'mirror'"  >> ~/.cargo/config \
    && echo '[source.mirror]' >> ~/.cargo/config \
    && echo 'registry = "https://mirrors.tuna.tsinghua.edu.cn/git/crates.io-index.git"'  >> ~/.cargo/config 

RUN cat ~/.cargo/config 

COPY . .

RUN sed -i "s@archive.ubuntu.com@mirrors.tuna.tsinghua.edu.cn@g" /etc/apt/sources.list && \
    sed -i "s@security.ubuntu.com@mirrors.tuna.tsinghua.edu.cn@g" /etc/apt/sources.list

RUN apt-get update && apt-get install -y libssl-dev pkg-config

RUN cargo build --release

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y libc6 libssl-dev pkg-config

WORKDIR /usr/app

COPY --from=builder /usr/src/config /usr/app/config
COPY --from=builder /usr/src/target/release/credits-cli /usr/app/credits-cli

ENV start_params=" "

EXPOSE 8080
CMD ["sh", "-c", "/usr/app/credits-cli task ${task_params} && /usr/app/credits-cli start"]