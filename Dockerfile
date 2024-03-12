FROM rust:1.74-slim as builder

WORKDIR /usr/src/

COPY . .

# RUN sed -i "s@http://deb.debian.org@http://mirrors.aliyun.com@g" /etc/apt/sources.list
# RUN cat /etc/apt/sources.list
# RUN rm -Rf /var/lib/apt/lists/*
# RUN apt-get update

COPY dockerfileconfig/sources.list /etc/apt/sources.list

RUN cat /etc/apt/sources.list
RUN rm -Rf /var/lib/apt/lists/*
RUN apt-get update

RUN apt-get install -y libssl-dev pkg-config

COPY dockerfileconfig/config ~/.cargo/config 

RUN cargo build --release

FROM debian:bookworm-slim


COPY --from=builder /usr/src/dockerfileconfig/sources.list /etc/apt/sources.list
RUN cat /etc/apt/sources.list
RUN rm -Rf /var/lib/apt/lists/*
RUN apt-get update
# RUN sed -i "s@http://deb.debian.org@http://mirrors.aliyun.com@g" /etc/apt/sources.list && rm -Rf /var/lib/apt/lists/* && apt-get update

RUN apt-get install -y libc6 

WORKDIR /usr/app

COPY --from=builder /usr/src/config /usr/app/config
COPY --from=builder /usr/src/target/release/credits-cli /usr/app/credits-cli


ENV task_params=" "

EXPOSE 8080
CMD ["sh", "-c", "./credits-cli task $task_params && ./credits-cli start"]


# # ENV start_params=" "


# # FROM dockerhub.tezign.com/innovation/muse-credits/muse-credits:v1.6

# # ENV start_params=" "

# # EXPOSE 8080
# # CMD ["sh", "-c", "/usr/app/credits-cli task ${task_params} && /usr/app/credits-cli start"]

# FROM dockerhub.tezign.com/tekton/cuda:11.7.0-cudnn8-runtime-ubuntu22.04
# # RUN sed -i "s@archive.ubuntu.com@mirrors.tuna.tsinghua.edu.cn@g" /etc/apt/sources.list && \
# #     sed -i "s@security.ubuntu.com@mirrors.tuna.tsinghua.edu.cn@g" /etc/apt/sources.list

# RUN apt-get update && \
#     apt-get install -y curl build-essential libssl-dev pkg-config

# RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

# ENV PATH="/root/.cargo/bin:${PATH}"

# WORKDIR /usr/app/

# COPY . .

# RUN echo '[source.crates-io]' > ~/.cargo/config \
#     && echo "replace-with = 'ustc'"  >> ~/.cargo/config \
#     && echo '[source.ustc]' >> ~/.cargo/config \
#     && echo 'registry = "git://mirrors.ustc.edu.cn/crates.io-index"'  >> ~/.cargo/config 

# RUN cargo build --release

# ENV start_params " "

# EXPOSE 8080
# CMD ["sh", "-c", "./credits-cli task $task_params && ./credits-cli start"]
