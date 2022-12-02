#!/bin/sh

#set env variables during debugging
APP_GIT_REPO="10000"
APP_GIT_BRANCH="2"
APP_GIT_COMMIT="3"
APP_GIT_TAG="4"
APP_GITHUB_WORKFLOW="5"
APP_GITHUB_RUN_ID="6"
APP_GITHUB_RUN_NUMBER="7"

RUST_LOG=debug

cargo run