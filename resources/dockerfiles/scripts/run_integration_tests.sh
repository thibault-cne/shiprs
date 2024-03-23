#!/bin/sh

# Check if docker is running
if ! docker info > /dev/null 2>&1; then
  echo "Docker is not running"
  exit 1
fi

export REGISTRY_PASSWORD=$(date | md5sum | cut -f1 -d\ )
docker create -v /etc/docker/registry --name config alpine:3 /bin/true
echo -n "${REGISTRY_PASSWORD}" | docker run --rm -i --entrypoint=htpasswd --volumes-from config thibaultcne/htpasswd -i -B -c /etc/docker/registry/htpasswd shiprs
cat resources/dockerfiles/registry/config.yml | docker run --rm -i --volumes-from config --entrypoint=tee alpine:3 /etc/docker/registry/config.yml
docker run -d --restart always --name registry -p 5000:5000 --volumes-from config registry:2
sleep 4

# Pull images from the docker.io registry
docker pull hello-world:linux
docker pull alpine
docker pull androw/uhttpd

# Tag images to then push them to the local registry
docker tag hello-world:linux localhost:5000/hello-world:linux
docker tag alpine localhost:5000/alpine
docker tag androw/uhttpd localhost:5000/androw/uhttpd

# Login to registry
docker login --username shiprs --password "${REGISTRY_PASSWORD}" localhost:5000

# Push images to the local registry
docker push localhost:5000/hello-world:linux
docker push localhost:5000/alpine
docker push localhost:5000/androw/uhttpd

# Run tests
docker swarm init
docker run -e REGISTRY_PASSWORD -e REGISTRY_HTTP_ADDR=localhost:5000 -v /var/run/docker.sock:/var/run/docker.sock -ti --rm shiprs cargo test $@ -- --test-threads 1
