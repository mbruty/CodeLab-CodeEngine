clear
echo "Building code engine"
cargo build --release
echo "Starting build for all images"
for i in "node"
do
    echo "Building $i"
    docker build -f ./docker/$i.dockerfile -t $i-engine .
done