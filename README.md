# Multi-Architecture Container Image w/Rust

## Introduction

I've been developing a service orientated smart home system which consists of a number of containerised workloads running on an edge Kubernetes cluster (via [Microk8s](https://github.com/canonical/microk8s)), the "cluster" itself is a sole Raspberry Pi 4b (ARMv8).

As well as running multiple workloads on the Pi 4b I also run workloads on another Raspberry Pi 2b (ARMv7) which is much older (but very power efficient). And finally I also need to run general tests of the workloads on my local Windows development machine prior to deployment to my "Production cluster", and at a later date I may even want to run these workloads on [Azure Kubernetes Service](https://azure.microsoft.com/en-us/products/kubernetes-service/).

Although I could achieve my goal of deploying the same application to multiple architectures using separate Dockerfiles (i.e. Dockerfile.amd64, Dockerfile.arm64, etc...) in my view that is messy and makes the CI/CD more complex. I think the single Dockerfile is the elegant approach keeping all build instructions in one place.

This repository provides a fully working example of building a Rust application container image that is capable of targeting multiple platform architectures - all from a single Dockerfile.

If you find this repository useful then give it a :star: ... :wink:

## Goals

- Construct a Rust multi-architecture container image via a single Dockerfile using the `docker buildx` command.
- Create a single GitHub Actions workflow [ci.yml](.github/workflows/ci.yml) to handle all tasks and host the reuseable workflows in an external [gha-workflows](https://github.com/f2calv/gha-workflows) repository.

  - Auto-Semantic Versioning
  - Build App
  - Build Container + Push To GitHub Packages
  - Package Helm Chart + Push To GitHub Packages
  - GitHub Release

## Run Pre-Built Container Image

```bash
#Run pre-built image on Docker
docker run --pull always --rm -it ghcr.io/f2calv/multi-arch-container-rust

#Run pre-built image on Kubernetes (via Helm)
helm upgrade --install multi-arch-container-rust oci://ghcr.io/f2calv/charts/multi-arch-container-rust
#helm uninstall multi-arch-container-rust

#Run pre-built image on Kubernetes (via kubectl)
kubectl run -i --tty --attach multi-arch-container-rust --image=ghcr.io/f2calv/multi-arch-container-rust --image-pull-policy='Always'
kubectl logs -f multi-arch-container-rust
#kubectl delete po multi-arch-container-rust
```

## Self-Build Container Image Locally

The Rust workload is an ultra simple worker process (i.e. a console application) which loops outputting a number of environment variables passed in during the CI process and then baked into the container image.

First clone the repository (ideally by opening it as [vscode devcontainer](https://marketplace.visualstudio.com/items?itemName=ms-vscode-remote.remote-containers)) and then via a terminal window from the root of the repository execute;

  ```powershell
  #demo script PowerShell version
  ./build.ps1
  ```
  
  Or
  
  ```bash
  #demo script Shell version (also below)
  . build.sh
  ```

### Shell Demo Script

```bash
#!/bin/sh

#set variables to emulate running in the workflow/pipeline
GIT_REPOSITORY=$(basename `git rev-parse --show-toplevel`)
GIT_BRANCH=$(git branch --show-current)
GIT_COMMIT=$(git rev-parse HEAD)
GIT_TAG="latest-dev"
GITHUB_WORKFLOW="n/a"
GITHUB_RUN_ID=0
GITHUB_RUN_NUMBER=0
IMAGE_NAME="$GIT_REPOSITORY:$GIT_TAG"
#Note: you cannot export a buildx container image into a local docker instance with multiple architecture manifests so for local testing you have to select just a single architecture.
#$PLATFORM="linux/amd64,linux/arm64,linux/arm/v7"
PLATFORM="linux/amd64"

#Create a new builder instance
#https://github.com/docker/buildx/blob/master/docs/reference/buildx_create.md
docker buildx create --name multiarchcontainerrust --use

#Start a build
#https://github.com/docker/buildx/blob/master/docs/reference/buildx_build.md
docker buildx build \
    -t $IMAGE_NAME \
    --label "GITHUB_RUN_ID=$GITHUB_RUN_ID" \
    --label "IMAGE_NAME=$IMAGE_NAME" \
    --build-arg GIT_REPOSITORY=$GIT_REPOSITORY \
    --build-arg GIT_BRANCH=$GIT_BRANCH \
    --build-arg GIT_COMMIT=$GIT_COMMIT \
    --build-arg GIT_TAG=$GIT_TAG \
    --build-arg GITHUB_WORKFLOW=$GITHUB_WORKFLOW \
    --build-arg GITHUB_RUN_ID=$GITHUB_RUN_ID \
    --build-arg GITHUB_RUN_NUMBER=$GITHUB_RUN_NUMBER \
    --platform $PLATFORM \
    --pull \
    -o type=docker \
    .

#Preview matching images
#https://docs.docker.com/engine/reference/commandline/images/
docker images $GIT_REPOSITORY

read -p "Hit ENTER to run the '$IMAGE_NAME' image..."

#Run the multi-architecture container image
#https://docs.docker.com/engine/reference/commandline/run/
docker run --rm -it --name $GIT_REPOSITORY $IMAGE_NAME

#userprofile=$(wslpath "$(wslvar USERPROFILE)")
#export KUBECONFIG=$userprofile/.kube/config
#kubectl run -i --tty --attach multi-arch-container-rust --image=gcr.io/f2calv/multi-arch-container-rust --image-pull-policy='Always'
```

## Docker, Container & Rust Resources

- I highly recommend reading the official Docker blog posts about multi-arch images;

  - https://www.docker.com/blog/multi-arch-images/
  - https://www.docker.com/blog/multi-arch-build-and-images-the-simple-way/
  - https://www.docker.com/blog/faster-multi-platform-builds-dockerfile-cross-compilation-guide/
  - https://www.docker.com/blog/cross-compiling-rust-code-for-multiple-architectures/

- Official Docker documentation about support/implementation for multi-arch images;

  - https://github.com/docker/buildx
  - https://docs.docker.com/desktop/multi-arch/
  - https://docs.docker.com/buildx/working-with-buildx/
  - https://docs.docker.com/engine/reference/commandline/buildx_build/

- Official Rust documentation useful for multi-arch builds;

  - https://rust-lang.github.io/rustup/cross-compilation.html
  - https://doc.rust-lang.org/nightly/rustc/platform-support.html

## Other Resources

- [Click here for .NET version of this repository...](https://github.com/f2calv/multi-arch-container-dotnet)
