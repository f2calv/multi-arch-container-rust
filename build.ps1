Set-StrictMode -Version 3.0
$ErrorActionPreference = "Stop"

#set variables to emulate running in the workflow/pipeline
$REPO_ROOT = git rev-parse --show-toplevel
$GIT_REPOSITORY = $REPO_ROOT | Split-Path -Leaf
$GIT_BRANCH = $(git branch --show-current)
$GIT_COMMIT = $(git rev-parse HEAD)
$GIT_TAG = "latest-dev"
$GITHUB_WORKFLOW = "n/a"
$GITHUB_RUN_ID = 0
$GITHUB_RUN_NUMBER = 0
$IMAGE_NAME = "$($GIT_REPOSITORY):$($GIT_TAG)"
#Note: you cannot export a buildx container image into a local docker instance with multiple architecture manifests so for local testing you have to select just a single architecture.
#$PLATFORM = "linux/amd64,linux/arm64,linux/arm/v7"
$PLATFORM = "linux/amd64"

#Create a new builder instance
#https://github.com/docker/buildx/blob/master/docs/reference/buildx_create.md
docker buildx create --name multiarchcontainerrust --use

#Start a build
#https://github.com/docker/buildx/blob/master/docs/reference/buildx_build.md
docker buildx build `
    -t $IMAGE_NAME `
    --build-arg GIT_REPOSITORY=$GIT_REPOSITORY `
    --build-arg GIT_BRANCH=$GIT_BRANCH `
    --build-arg GIT_COMMIT=$GIT_COMMIT `
    --build-arg GIT_TAG=$GIT_TAG `
    --build-arg GITHUB_WORKFLOW=$GITHUB_WORKFLOW `
    --build-arg GITHUB_RUN_ID=$GITHUB_RUN_ID `
    --build-arg GITHUB_RUN_NUMBER=$GITHUB_RUN_NUMBER `
    --platform $PLATFORM `
    --pull `
    -o type=docker `
    .

#Preview matching images
#https://docs.docker.com/engine/reference/commandline/images/
docker images $GIT_REPOSITORY

Write-Host "Hit ENTER to run the '$IMAGE_NAME' image..." 
pause

#Run the multi-architecture container image
#https://docs.docker.com/engine/reference/commandline/run/
docker run --rm -it --name $GIT_REPOSITORY $IMAGE_NAME

#kubectl run -i --tty --attach multi-arch-container-rust --image=ghcr.io/f2calv/multi-arch-container-rust --image-pull-policy='Always'