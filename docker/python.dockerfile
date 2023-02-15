FROM ubuntu
ENV DEBIAN_FRONTEND=noninteractive
RUN apt-get update
RUN apt-get install procps -y
RUN apt-get install sysstat -y
RUN apt-get install software-properties-common -y
RUN add-apt-repository ppa:deadsnakes/ppa
RUN apt-get update
RUN apt-get install python3.8 -y
RUN apt install python3-pip	-y
RUN pip install numpy
RUN pip install pandas
RUN pip install matplotlib
COPY ./setup-code/python ./unsafe
COPY ./target/release/code_engine_rust .
ENTRYPOINT ["./code_engine_rust", "--language=python"]
