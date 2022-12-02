#!/bin/sh

#set env variables during debugging
APP_GIT_REPO=$(basename `git rev-parse --show-toplevel`)
APP_GIT_BRANCH=$(git branch --show-current)
APP_GIT_COMMIT=$(git rev-parse HEAD)
APP_GIT_TAG="n/a"
APP_GITHUB_WORKFLOW="n/a"
APP_GITHUB_RUN_ID=0
APP_GITHUB_RUN_NUMBER=0

RUST_LOG=debug

cargo run