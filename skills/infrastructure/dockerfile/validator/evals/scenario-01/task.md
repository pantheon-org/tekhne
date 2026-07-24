# Scenario: Flags :latest base tag and root user via hadolint

A Dockerfile uses FROM node:latest and never sets USER. Validate it and report the issues.
