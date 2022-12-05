#!/bin/sh

#set env variables during debugging
export REGISTRY="192.168.1.245:32000"
export GIT_REPO=$(basename `git rev-parse --show-toplevel`)
export GIT_BRANCH=$(git branch --show-current)
export GIT_COMMIT=$(git rev-parse HEAD)
#export GIT_TAG="dev"
export GIT_TAG="registry"
export GITHUB_WORKFLOW="n/a"
export GITHUB_RUN_ID=0
export GITHUB_RUN_NUMBER=0

export RUST_LOG=debug

#RUST_LOG=debug cargo run

DOCKER_BUILDKIT=1

docker buildx create --name multiarchrust --use --config microk8s.toml

docker buildx build -t $REGISTRY/multi-arch-container-rust:$GIT_TAG \
    --build-arg GIT_REPO=$GIT_REPO \
    --build-arg GIT_BRANCH=$GIT_BRANCH \
    --build-arg GIT_COMMIT=$GIT_COMMIT \
    --build-arg GIT_TAG=$GIT_TAG \
    --build-arg GITHUB_WORKFLOW=$GITHUB_WORKFLOW \
    --build-arg GITHUB_RUN_ID=$GITHUB_RUN_ID \
    --build-arg GITHUB_RUN_NUMBER=$GITHUB_RUN_NUMBER \
    --platform linux/amd64,linux/arm64,linux/arm/v7 \
    --pull \
    --push \
    .

#docker run --rm -it --name test multi-arch-container-rust:$GIT_TAG -e RUST_LOG=debug

#docker compose up --build --remove-orphans
