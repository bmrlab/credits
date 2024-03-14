FROM rust:1.74-slim

WORKDIR /usr/src/

COPY . .

# debian  source.list 地址换到了 /etc/apt/sources.list.d/debian.sources
RUN sed -i "s@http://deb.debian.org@https://mirrors.tuna.tsinghua.edu.cn@g" /etc/apt/sources.list.d/debian.sources
RUN rm -Rf /var/lib/apt/lists/*
RUN apt-get update

RUN apt-get install -y libssl-dev pkg-config

RUN cargo build --release

ENV task_params=" "

COPY  /usr/src/target/release/credits-cli /usr/src/credits-cli

EXPOSE 8080
CMD ["sh", "-c", "./credits-cli task $task_params && ./credits-cli start"]

# FROM debian:bookworm-slim

# # debian:bookworm-slim 清华大学数据源没有证书，需要安装证书麻烦。 使用科大源
# RUN sed -i 's/deb.debian.org/mirrors.ustc.edu.cn/g' /etc/apt/sources.list.d/debian.sources
# RUN rm -Rf /var/lib/apt/lists/*
# RUN apt-get update
# RUN apt-get install -y libssl-dev pkg-config ca-certificates

# WORKDIR /usr/app

# COPY --from=builder /usr/src/config /usr/app/config
# COPY --from=builder /usr/src/target/release/credits-cli /usr/app/credits-cli


# ENV task_params=" "

# EXPOSE 8080
# CMD ["sh", "-c", "./credits-cli task $task_params && ./credits-cli start"]


