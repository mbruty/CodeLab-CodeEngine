FROM python
RUN apt-get update
RUN apt-get install procps -y
RUN apt-get install sysstat -y
COPY ./target/release/code_engine_rust .
ENTRYPOINT ["./code_engine_rust", "--language=python"]
