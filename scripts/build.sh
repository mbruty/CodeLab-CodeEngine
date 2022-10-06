clear
echo "Starting build of code_engine"
cargo build --release
echo "Starting build for all images"
for i in "bun" "dotnet" "typescript"
do
    echo "Building $i"
    docker build -f ./docker/$i.dockerfile -t $i-engine .
done
