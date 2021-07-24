FROM ubuntu
RUN apt-get update
RUN apt-get upgrade -y
RUN DEBIAN_FRONTEND="noninteractive" apt-get install curl git build-essential libssl-dev openssl pkg-config ca-certificates --assume-yes --fix-missing
RUN ldconfig
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y
COPY . /home/build
ENV PATH=$PATH:$HOME/.cargo/bin
RUN cd /home/build; $HOME/.cargo/bin/cargo build --release

FROM ubuntu
RUN apt-get update; apt-get upgrade -y; DEBIAN_FRONTEND="noninteractive" apt-get install libssl-dev openssl ca-certificates --assume-yes --fix-missing
WORKDIR /root/
COPY --from=0 /home/build/target/release/sneedbot ./
CMD ["./sneedbot"]