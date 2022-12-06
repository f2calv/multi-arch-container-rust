Set-StrictMode -Version 3.0
$ErrorActionPreference = "Stop"

kubectl delete po aarch64test

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
# & "docker" buildx create --name multiarchrust --use --config microk8s.toml

# & "docker" buildx build `
#     -t "$REGISTRY/aarch64:$GIT_TAG" `
#     -f Dockerfile.aarch64 `
#     --build-arg TARGET=aarch64-unknown-linux-gnu `
#     --build-arg GIT_REPO=$GIT_REPO `
#     --build-arg GIT_TAG=$GIT_TAG `
#     --build-arg GIT_BRANCH=$GIT_BRANCH `
#     --build-arg GIT_COMMIT=$GIT_COMMIT `
#     --build-arg GITHUB_WORKFLOW=$GITHUB_WORKFLOW `
#     --build-arg GITHUB_RUN_ID=$GITHUB_RUN_ID `
#     --build-arg GITHUB_RUN_NUMBER=$GITHUB_RUN_NUMBER `
#     --platform linux/arm64 `
#     --pull `
#     --push `
#     .

& "docker" build `
    -t "$REGISTRY/aarch64:$GIT_TAG" `
    -f Dockerfile.aarch64 `
    --build-arg TARGET=aarch64-unknown-linux-gnu `
    --build-arg GIT_REPO=$GIT_REPO `
    --build-arg GIT_TAG=$GIT_TAG `
    --build-arg GIT_BRANCH=$GIT_BRANCH `
    --build-arg GIT_COMMIT=$GIT_COMMIT `
    --build-arg GITHUB_WORKFLOW=$GITHUB_WORKFLOW `
    --build-arg GITHUB_RUN_ID=$GITHUB_RUN_ID `
    --build-arg GITHUB_RUN_NUMBER=$GITHUB_RUN_NUMBER `
    --platform linux/arm64 `
    --progress=plain `
    --pull `
    .

#docker push "$REGISTRY/aarch64:$GIT_TAG"
#docker run --rm -it --name temp 192.168.1.245:32000/aarch64:registry /bin/bash

#kubectl run -i --tty --attach aarch64test --image=192.168.1.245:32000/aarch64:registry