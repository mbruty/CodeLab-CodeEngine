clear
echo "Starting deployment for all images"
for i in "dotnet-engine" "bun-engine" "typescript-engine" "python-engine"
do
    echo "Pulling $i"
    docker pull ghcr.io/mbruty/mike-codelab-codeengine/$i:latest
done
echo "Pulling complete"
echo "Killing old containers"
docker stop $(docker ps -a -q)
echo "Pruning old images"
docker image prune -a -f
echo "Done"
