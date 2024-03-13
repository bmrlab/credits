FROM rust:1.74-slim as builder

WORKDIR /usr/src/

COPY . .

RUN sed -i "s@http://deb.debian.org@https://mirrors.tuna.tsinghua.edu.cn@g" /etc/apt/sources.list.d/debian.sources
RUN cat /etc/apt/sources.list.d/debian.sources
RUN rm -Rf /var/lib/apt/lists/*
RUN apt-get update

RUN apt-get install -y libssl-dev pkg-config

RUN cargo build --release

FROM debian:bookworm-slim

RUN sed -i "s@http://deb.debian.org@https://mirrors.tuna.tsinghua.edu.cn@g" /etc/apt/sources.list.d/debian.sources
RUN cat /etc/apt/sources.list.d/debian.sources
RUN rm -Rf /var/lib/apt/lists/*
RUN apt-get update


WORKDIR /usr/app

COPY --from=builder /usr/src/config /usr/app/config
COPY --from=builder /usr/src/target/release/credits-cli /usr/app/credits-cli


ENV task_params=" "

EXPOSE 8080
CMD ["sh", "-c", "./credits-cli task $task_params && ./credits-cli start"]