#!/bin/sh

#set env variables during debugging
export GIT_REPO=$(basename `git rev-parse --show-toplevel`)
export GIT_BRANCH=$(git branch --show-current)
export GIT_COMMIT=$(git rev-parse HEAD)
export GIT_TAG="n/a"
export GITHUB_WORKFLOW="n/a"
export GITHUB_RUN_ID=0
export GITHUB_RUN_NUMBER=0

DOCKER_BUILDKIT=0
export RUST_LOG=debug
#cargo run
docker compose up --build --remove-orphans
