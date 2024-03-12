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

RUN echo '[source.crates-io]' > ~/.cargo/config \
    && echo "replace-with = 'ustc'"  >> ~/.cargo/config \
    && echo '[source.ustc]' >> ~/.cargo/config \
    && echo 'registry = "git://mirrors.ustc.edu.cn/crates.io-index"'  >> ~/.cargo/config 



RUN cargo build --release


ENV start_params " "

EXPOSE 8080
CMD ["sh", "-c", "/usr/src/target/release/credits-cli task $task_params && /usr/src/target/release/credits-cli start"]
