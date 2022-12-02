#!/bin/sh

#set env variables during debugging
export GIT_REPO=$(basename `git rev-parse --show-toplevel`)
export GIT_BRANCH=$(git branch --show-current)
export GIT_COMMIT=$(git rev-parse HEAD)
export GIT_TAG="n/a"
export GITHUB_WORKFLOW="n/a"
export GITHUB_RUN_ID=0
export GITHUB_RUN_NUMBER=0

export RUST_LOG=debug

#RUST_LOG=debug cargo run

DOCKER_BUILDKIT=0

docker build -t multi-arch-container-rust:dev \
    --build-arg GIT_REPO=$GIT_REPO \
    --build-arg GIT_BRANCH=$GIT_BRANCH \
    --build-arg GIT_COMMIT=$GIT_COMMIT \
    --build-arg GIT_TAG=$GIT_TAG \
    --build-arg GITHUB_WORKFLOW=$GITHUB_WORKFLOW \
    --build-arg GITHUB_RUN_ID=$GITHUB_RUN_ID \
    --build-arg GITHUB_RUN_NUMBER=$GITHUB_RUN_NUMBER \
    --pull \
    .

#docker run --rm -it --name test multi-arch-container-rust:dev -e RUST_LOG=debug

docker compose up --build --remove-orphans
