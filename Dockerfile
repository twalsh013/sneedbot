FROM ubuntu
RUN apt-get update
RUN apt-get upgrade -y
COPY ./target/aarch64-unknown-linux-gnu/debug/sneedbot /root/
CMD /root/sneedbot