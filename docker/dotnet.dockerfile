FROM mcr.microsoft.com/dotnet/sdk:6.0
RUN apt-get update
RUN apt-get install procps -y
RUN apt-get install sysstat -y
COPY ./warmup-code/dotnet ./unsafe
RUN dotnet build ./unsafe --configuration Release
COPY ./target/release/code_engine_rust .
RUN rm ./unsafe/Solution.cs
RUN rm ./unsafe/UnitTestsSolution.cs
RUN rm ./unsafe/bin/Release/net6.0/Application
ENTRYPOINT ["./code_engine_rust", "--language=dotnet"]