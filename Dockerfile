# FROM dockerhub.tezign.com/innovation/muse-credits/muse-credits:v1.6

# ENV start_params=" "

# EXPOSE 8080
# CMD ["sh", "-c", "/usr/app/credits-cli task ${task_params} && /usr/app/credits-cli start"]

FROM dockerhub.tezign.com/tekton/cuda:11.7.0-cudnn8-runtime-ubuntu22.04
# RUN sed -i "s@archive.ubuntu.com@mirrors.tuna.tsinghua.edu.cn@g" /etc/apt/sources.list && \
#     sed -i "s@security.ubuntu.com@mirrors.tuna.tsinghua.edu.cn@g" /etc/apt/sources.list

RUN apt-get update && \
    apt-get install -y curl build-essential libssl-dev pkg-config

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"
COPY . .
WORKDIR /usr/src/

RUN mkdir ~/.cargo/ && touch ~/.cargo/config \
    && echo '[source.crates-io]' > ~/.cargo/config \
    && echo "replace-with = 'mirror'"  >> ~/.cargo/config \
    && echo '[source.mirror]' >> ~/.cargo/config \
    && echo 'registry = "https://mirrors.tuna.tsinghua.edu.cn/git/crates.io-index.git"'  >> ~/.cargo/config 



RUN cargo build --release

# COPY config /usr/app/config
# COPY target/release/credits-cli /usr/app/credits-cli

ENV start_params " "

EXPOSE 8080
CMD ["sh", "-c", "./target/release/credits-cli task $task_params && ./target/release/credits-cli start"]
