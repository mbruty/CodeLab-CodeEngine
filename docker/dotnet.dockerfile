FROM ubuntu
RUN apt-get update --fix-missing
RUN apt-get install wget -y
RUN wget https://packages.microsoft.com/config/ubuntu/22.10/packages-microsoft-prod.deb -O packages-microsoft-prod.deb
RUN dpkg -i packages-microsoft-prod.deb
RUN rm packages-microsoft-prod.deb
RUN apt-get update
RUN apt-get install -y bash curl unzip procps sysstat dotnet-sdk-7.0
COPY ./warmup-code/dotnet ./unsafe
RUN dotnet build ./unsafe --configuration Release
COPY ./target/release/code_engine_rust .
RUN rm ./unsafe/Solution.cs
RUN rm ./unsafe/UnitTests.cs
RUN rm ./unsafe/bin/Release/net6.0/Application
ENTRYPOINT ["./code_engine_rust", "--language=dotnet"]