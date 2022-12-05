#!/bin/sh

export KUBECONFIG=/mnt/c/Users/Alex/.kube/config

docker build -t 192.168.1.245:32000/x86_64:registry -f Dockerfile.x86_64 --build-arg TARGET=x86_64-unknown-linux-gnu .

#docker run --rm -it --name temp 192.168.1.245:32000/x86_64:registry
#docker run --rm -it --name temp 192.168.1.245:32000/x86_64:registry --entrypoint=sh
#docker run --rm -it --name temp 192.168.1.245:32000/x86_64:registry /bin/bash

#docker push 192.168.1.245:32000/x86_64:registry

#kubectl delete -f x86_64.yaml
#kubectl apply -f x86_64.yaml
#kubectl get po -w
