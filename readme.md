
# CodeLab - CodeEngine
## Navigation
[Website](https://github.com/mbruty/mike-CodeLab-Web)

[API](https://github.com/mbruty/mike-CodeLab-Graphql)

CodeEngine

[Docker Scheduler](https://github.com/mbruty/mike-CodeLab-Scheduler)

## Project Vision
Many software developers rely on online coding platforms, such as LeetCode, to enhance their knowledge. These platforms have been successful with students who want to expand their learning beyond what is taught. These platforms could also be helpful in educational institutions to teach software development. However, the current platform’s lack the ability for teachers to create tasks and courses, and they do not provide enough insight into the effects of optimisations on the users' code.

The CodeLab platform aims to bring online coding platforms to the education sector. CodeLab offers the same features as the majority of online coding platforms. CodeLab also offers a way for teachers to create tasks, and group the into modules. For students, CodeLab offers more detailed utilisation statistics than other platform. CodeLab also does not limit usage to only algorithm-style questions and facilitates a wider bredth of teaching.
## Development Setup
Ensure that you have Cargo v1.63.0 or newer installed.
**Note: This project only works on Linux**

### Install Linux dependencies
```bash
apt-get update
apt-get install -y bash curl unzip procps sysstat
```

#### Dotnet
```bash
wget https://packages.microsoft.com/config/ubuntu/22.10/packages-microsoft-prod.deb -O packages-microsoft-prod.deb
dpkg -i packages-microsoft-prod.deb
rm packages-microsoft-prod.deb
apt-get update
apt-get install -y dotnet-sdk-7.0
```

#### JavaScript | TypeScript
```bash
curl https://bun.sh/install | bash
```

#### Python
```bash
apt-get install python3.8 -y
apt install python3-pip	-y
pip install numpy pandas matplotlib
```

### Compilation and running
1. Clone the repository
2. Run `cargo build --release`
3. Run `./target/debug/code_engine_rust --language=<the language you want to run>`
4. If you are not running the API locally, change the URL in config.ts to "https://gql.bruty.net"


## Setup
Ensure that you have yout GitHub personal-access-token linked to docker.

### Docker
#### Dotnet
Run `docker pull ghcr.io/mbruty/mike-codelab-codeengine/dotnet-engine`

Run `docker run --rm ghcr.io/mbruty/mike-codelab-codeengine/dotnet-engine`

#### JavaScript
Run `docker pull ghcr.io/mbruty/mike-codelab-codeengine/bun-engine`

Run `docker run --rm ghcr.io/mbruty/mike-codelab-codeengine/bun-engine`

#### TypeScript
Run `docker pull ghcr.io/mbruty/mike-codelab-codeengine/typescript-engine`

Run `docker run --rm ghcr.io/mbruty/mike-codelab-codeengine/typescript-engine`


#### Python
Run `docker pull ghcr.io/mbruty/mike-codelab-codeengine/python-engine`

Run `docker run --rm ghcr.io/mbruty/mike-codelab-codeengine/python-engine`

## Technologies uesd
|Name|Version|
|--|--|
|Bun|0.5.9|
|Cargo|1.63.0|
|Dotnet|7.0|
|Python|3.8|
|amiquip|0.4|
|openssl|0.10|
|redis|0.23.0|
|serde|1.0.162|
|serde_json|1.0.90|
|serde_derive|1.0.162|

