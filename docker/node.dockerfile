FROM node:current-alpine3.16
RUN pwd
RUN apk update
RUN apk add procps
RUN apk add sysstat
COPY ./target/release/code_engine_rust .
RUN ls
ENTRYPOINT ["./code_engine_rust", "--language=node"]