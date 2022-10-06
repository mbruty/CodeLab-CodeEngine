clear
echo "Starting deployment for all images"
for i in "dotnet-engine" "bun-engine" "typescript-engine"
do
    echo "Deploying $i"
    docker tag $i europe-west3-docker.pkg.dev/finalyearproject-363115/code-engine/$i
    docker push europe-west3-docker.pkg.dev/finalyearproject-363115/code-engine/$i
done
