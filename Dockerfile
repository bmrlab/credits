FROM dockerhub.tezign.com/tekton/cuda:11.7.0-cudnn8-runtime-ubuntu22.04

RUN apt-get update && \
    apt-get install -y curl build-essential libssl-dev pkg-config

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

ENV PATH="/root/.cargo/bin:${PATH}"

WORKDIR /usr/app/

COPY . .

RUN echo '[source.crates-io]' > ~/.cargo/config \
    && echo "replace-with = 'ustc'"  >> ~/.cargo/config \
    && echo '[source.ustc]' >> ~/.cargo/config \
    && echo 'registry = "git://mirrors.ustc.edu.cn/crates.io-index"'  >> ~/.cargo/config 

RUN cargo build --release

ENV task_params " "

EXPOSE 8080
CMD ["sh", "-c", "./target/release/credits-cli task $task_params & ./target/release/credits-cli start"]
