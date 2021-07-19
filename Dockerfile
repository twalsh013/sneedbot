FROM ubuntu
RUN apt-get update
RUN apt-get upgrade -y
RUN apt-get install openssl ca-certificates libssl-dev --assume-yes
RUN ldconfig
COPY ./goodsneed /root/sneedbot
CMD /root/sneedbot