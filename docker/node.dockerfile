FROM node:16.17.1
RUN apt-get update
RUN apt-get install procps -y
RUN apt-get install sysstat -y
COPY ./target/release/code_engine_rust .
ENTRYPOINT ["./code_engine_rust", "--language=node"]