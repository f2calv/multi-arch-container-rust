Set-StrictMode -Version 3.0
$ErrorActionPreference = "Stop"

#kubectl delete po aarch64test

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
#     -t "$REGISTRY/x86_64:$GIT_TAG" `
#     -f Dockerfile.x86_64 `
#     --build-arg TARGET=x86_64-unknown-linux-gnu `
#     --build-arg GIT_REPO=$GIT_REPO `
#     --build-arg GIT_TAG=$GIT_TAG `
#     --build-arg GIT_BRANCH=$GIT_BRANCH `
#     --build-arg GIT_COMMIT=$GIT_COMMIT `
#     --build-arg GITHUB_WORKFLOW=$GITHUB_WORKFLOW `
#     --build-arg GITHUB_RUN_ID=$GITHUB_RUN_ID `
#     --build-arg GITHUB_RUN_NUMBER=$GITHUB_RUN_NUMBER `
#     --platform linux/amd64 `
#     --pull `
#     --push `
#     .

& "docker" build `
    -t "$REGISTRY/x86_64:$GIT_TAG" `
    -f Dockerfile.x86_64 `
    --build-arg TARGET=x86_64-unknown-linux-gnu `
    --build-arg GIT_REPO=$GIT_REPO `
    --build-arg GIT_TAG=$GIT_TAG `
    --build-arg GIT_BRANCH=$GIT_BRANCH `
    --build-arg GIT_COMMIT=$GIT_COMMIT `
    --build-arg GITHUB_WORKFLOW=$GITHUB_WORKFLOW `
    --build-arg GITHUB_RUN_ID=$GITHUB_RUN_ID `
    --build-arg GITHUB_RUN_NUMBER=$GITHUB_RUN_NUMBER `
    --platform linux/amd64 `
    --pull `
    .

#docker push "$REGISTRY/x86_64:$GIT_TAG"
#docker run --rm -it --name temp 192.168.1.245:32000/x86_64:registry /bin/bash
docker run --rm -it --name temp 192.168.1.245:32000/x86_64:registry
