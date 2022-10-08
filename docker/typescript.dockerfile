FROM ubuntu
RUN apt-get update
RUN apt-get install -y bash curl unzip procps sysstat
RUN curl https://bun.sh/install | bash
COPY ./target/release/code_engine_rust .
ENTRYPOINT ["./code_engine_rust", "--language=typescript"]
