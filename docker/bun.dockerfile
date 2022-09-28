FROM ubuntu
RUN apt-get update
RUN apt-get install -y bash curl unzip
RUN curl https://bun.sh/install | bash
RUN whereis bun
RUN apt-get install procps -y
RUN apt-get install sysstat -y
COPY ./target/release/code_engine_rust .
ENTRYPOINT ["./code_engine_rust", "--language=bun"]
