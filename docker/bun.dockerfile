FROM jarredsumner/bun:edge
RUN apk update
RUN apk add procps
RUN apk add sysstat
COPY ./target/release/code_engine_rust .
ENTRYPOINT ["./code_engine_rust", "--language=bun"]