# Build and Deploy a Containerized Service to AKS

## Problem/Feature Description

An e-commerce platform team is moving their product catalog service to Kubernetes on Azure (AKS). Currently they build a Docker image locally and push it manually to an Azure Container Registry. They want to automate this with a pipeline that:

1. Builds and pushes the Docker image to ACR
2. Deploys the new image to their AKS staging environment
3. After manual approval, deploys to production AKS

The team has had incidents where a production deployment pulled the wrong image because the tag was ambiguous. They want the pipeline to tag images in a way that makes every deployment traceable back to a specific build.

## Output Specification

Produce `azure-pipelines.yml` with a complete multi-stage pipeline covering the Docker build, staging deployment, and production deployment. The ACR service connection name is `myACR` and the image repository is `catalog-service`. The AKS service connections are `aks-staging` and `aks-production`. Kubernetes manifests are in `k8s/deployment.yml` and `k8s/service.yml`.
