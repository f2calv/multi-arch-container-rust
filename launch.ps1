Set-StrictMode -Version 3.0
$ErrorActionPreference = "Stop"

$REGISTRY = "192.168.1.245:32000"
$REPO_ROOT = git rev-parse --show-toplevel
$GIT_REPO = $REPO_ROOT | Split-Path -Leaf
$GIT_TAG = "registry"
$GIT_BRANCH = $(git branch --show-current)
$GIT_COMMIT = $(git rev-parse HEAD)
$GITHUB_WORKFLOW = "n/a"
$GITHUB_RUN_ID = 0
$GITHUB_RUN_NUMBER = 0

#https://github.com/docker/buildx/issues/94#issuecomment-534831828
& "docker" buildx create --name multiarchcontainerrust --use --config microk8s.toml
& "docker" buildx build `
    -t "$REGISTRY/multi-arch-container-rust:$GIT_TAG" `
    --build-arg GIT_REPO=$GIT_REPO `
    --build-arg GIT_TAG=$GIT_TAG `
    --build-arg GIT_BRANCH=$GIT_BRANCH `
    --build-arg GIT_COMMIT=$GIT_COMMIT `
    --build-arg GITHUB_WORKFLOW=$GITHUB_WORKFLOW `
    --build-arg GITHUB_RUN_ID=$GITHUB_RUN_ID `
    --build-arg GITHUB_RUN_NUMBER=$GITHUB_RUN_NUMBER `
    --platform linux/amd64,linux/arm64,linux/arm/v7 `
    --pull `
    --push `
    .
