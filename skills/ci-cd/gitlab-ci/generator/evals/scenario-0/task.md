# Node.js API CI Pipeline

## Problem/Feature Description

A small product team is launching a new Node.js REST API and needs a GitLab CI pipeline wired up before they merge their first feature. They previously worked at a company that used `only: master` in their pipelines and they have copy-pasted that pattern into a draft `.gitlab-ci.yml` they shared. A DevOps consultant has flagged that this approach is outdated and will cause subtle problems as the team adopts GitLab's newer features, but the team does not know the modern equivalent.

The pipeline should run on every push, but the deploy step should only trigger on the `main` branch. The team uses Node 20 and produces a `dist/` build folder. Tests generate a `coverage/` directory that should be available for download for 3 days, while the build artifact only needs to exist for the next stage and can be discarded sooner.

## Output Specification

Produce a `.gitlab-ci.yml` file for this pipeline. The pipeline should have at minimum: an install/build stage, a test stage, and a deploy stage. Do not include any hardcoded credentials or tokens in the YAML.
