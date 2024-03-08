FROM rust:1.74-slim as builder

WORKDIR /usr/src/
RUN mkdir ~/.cargo/ && touch ~/.cargo/config \
    && echo '[source.crates-io]' > ~/.cargo/config \
    && echo "replace-with = 'mirror'"  >> ~/.cargo/config \
    && echo '[source.mirror]' > ~/.cargo/config \
    && echo 'registry = "https://mirrors.tuna.tsinghua.edu.cn/git/crates.io-index.git"'  >> ~/.cargo/config 


COPY . .

RUN cargo build --release

FROM debian:bookworm-slim
ENV task_params=" " start_params=" "

RUN apt-get update && apt-get install -y libc6

WORKDIR /usr/app

COPY --from=builder /usr/src/config /usr/app/config
COPY --from=builder /usr/src/target/release/credits-cli /usr/app/credits-cli
EXPOSE 8080
CMD ["sh", "-c", "cd /usr/app && ./credits-cli task ${task_params} && ${start_params} ./credits-cli start"]
