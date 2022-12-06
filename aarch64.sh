#!/bin/sh

userprofile=$(wslpath "$(wslvar USERPROFILE)")
export KUBECONFIG=$userprofile/.kube/config

kubectl delete po aarch64test

docker buildx create --name multiarchrust2 --use --config microk8s.toml

docker buildx build \
    -t 192.168.1.245:32000/aarch64:registry \
    -f Dockerfile.aarch64 \
    --build-arg TARGET=aarch64-unknown-linux-gnu \
    --platform linux/arm64 \
    --pull \
    --push \
    .

#docker run --rm -it --name temp 192.168.1.245:32000/aarch64:registry
#docker run --rm -it --name temp 192.168.1.245:32000/aarch64:registry --entrypoint=sh
#docker run --rm -it --name temp 192.168.1.245:32000/aarch64:registry /bin/bash

#docker push 192.168.1.245:32000/aarch64:registry

#kubectl delete -f aarch64.yaml
#kubectl apply -f aarch64.yaml
#kubectl get po -w

#Now launch the pod on the cluster...
#kubectl run -i --tty --attach aarch64test --image=192.168.1.245:32000/aarch64:registry