#!/bin/sh

export KUBECONFIG=/mnt/c/Users/Alex/.kube/config

docker build -t 192.168.1.245:32000/aarch64:registry -f Dockerfile.aarch64 .

docker run --rm -it --name temp 192.168.1.245:32000/aarch64:registry --entrypoint=sh

docker push 192.168.1.245:32000/aarch64:registry

kubectl delete -f aarch64.yaml
kubectl apply -f aarch64.yaml
kubectl get po -w
