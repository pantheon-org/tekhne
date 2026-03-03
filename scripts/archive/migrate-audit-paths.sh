#!/usr/bin/env sh
# Migrate audit paths to match new skill structure

set -e

cd .context/audits || exit 1

migrate_audit() {
  old_path="$1"
  new_path="$2"
  
  if [ -d "$old_path" ]; then
    mkdir -p "$(dirname "$new_path")"
    git mv "$old_path" "$new_path"
    echo "✓ Migrated: $old_path → $new_path"
  fi
}

# CI-CD domain
migrate_audit "github-actions-generator" "ci-cd/github-actions/generator"
migrate_audit "github-actions-validator" "ci-cd/github-actions/validator"
migrate_audit "gitlab-ci-generator" "ci-cd/gitlab-ci/generator"
migrate_audit "gitlab-ci-validator" "ci-cd/gitlab-ci/validator"
migrate_audit "azure-pipelines-generator" "ci-cd/azure-pipelines/generator"
migrate_audit "azure-pipelines-validator" "ci-cd/azure-pipelines/validator"
migrate_audit "jenkinsfile-generator" "ci-cd/jenkinsfile/generator"
migrate_audit "jenkinsfile-validator" "ci-cd/jenkinsfile/validator"
migrate_audit "helm-generator" "ci-cd/helm/generator"
migrate_audit "helm-validator" "ci-cd/helm/validator"
migrate_audit "fluentbit-generator" "ci-cd/fluentbit/generator"
migrate_audit "fluentbit-validator" "ci-cd/fluentbit/validator"

# Infrastructure domain
migrate_audit "terraform-generator" "infrastructure/terraform/generator"
migrate_audit "terraform-validator" "infrastructure/terraform/validator"
migrate_audit "terragrunt-generator" "infrastructure/terragrunt/generator"
migrate_audit "terragrunt-validator" "infrastructure/terragrunt/validator"
migrate_audit "ansible-generator" "infrastructure/ansible/generator"
migrate_audit "ansible-validator" "infrastructure/ansible/validator"
migrate_audit "dockerfile-generator" "infrastructure/dockerfile/generator"
migrate_audit "dockerfile-validator" "infrastructure/dockerfile/validator"
migrate_audit "k8s-yaml-generator" "infrastructure/k8s-yaml/generator"
migrate_audit "k8s-yaml-validator" "infrastructure/k8s-yaml/validator"
migrate_audit "cfn-behavior-validator" "infrastructure/cfn/behavior-validator"
migrate_audit "cfn-template-compare" "infrastructure/cfn/template-compare"
migrate_audit "cdk-nag" "infrastructure/aws-cdk/cdk-nag"

# Repository management domain
migrate_audit "nx-workspace-patterns" "repository-mgmt/nx/workspace-patterns"
migrate_audit "nx-biome-integration" "repository-mgmt/nx/biome-integration"
migrate_audit "nx-bun-integration" "repository-mgmt/nx/bun-integration"
migrate_audit "nx-executors" "repository-mgmt/nx/executors"
migrate_audit "nx-generators" "repository-mgmt/nx/generators"
migrate_audit "nx-vite-integration" "repository-mgmt/nx/vite-integration"
migrate_audit "extending-nx-plugins" "repository-mgmt/nx/extending-plugins"

# Development domain
migrate_audit "bun-development" "development/bun-development"
migrate_audit "biome-complete" "development/biome-complete"
migrate_audit "typescript-advanced" "development/typescript-advanced"
migrate_audit "commanderjs" "development/commanderjs"
migrate_audit "bash-script-generator" "development/scripting/bash-script/generator"
migrate_audit "bash-script-validator" "development/scripting/bash-script/validator"
migrate_audit "makefile-generator" "development/scripting/makefile/generator"
migrate_audit "makefile-validator" "development/scripting/makefile/validator"

# Agentic harness domain
migrate_audit "opencode-config" "agentic-harness/opencode"
migrate_audit "agents-md" "agentic-harness/agents-md"

# Testing domain
migrate_audit "bdd-testing" "testing/bdd-testing"
migrate_audit "test-driven-development" "testing/test-driven-development"
migrate_audit "skill-quality-auditor" "testing/skill-quality-auditor"
migrate_audit "ui-debug-workflow" "testing/ui-debug-workflow"

# Software engineering domain
migrate_audit "software-design-principles" "software-engineering/software-design-principles"

# Observability domain
migrate_audit "promql-generator" "observability/promql/generator"
migrate_audit "promql-validator" "observability/promql/validator"
migrate_audit "logql-generator" "observability/logql-generator"
migrate_audit "k8s-debug" "observability/k8s-debug"

# Documentation domain
migrate_audit "markdown-authoring" "documentation/markdown-authoring"
migrate_audit "acceptance-criteria" "documentation/acceptance-criteria"
migrate_audit "conventional-commits" "documentation/conventional-commits"
migrate_audit "plain-english" "documentation/plain-english"
migrate_audit "journal-entry-creator" "documentation/journal-entry-creator"

# Package management domain
migrate_audit "mise-complete" "package-mgmt/mise-complete"

# Project management domain
migrate_audit "moscow-prioritization" "project-mgmt/moscow-prioritization"
migrate_audit "implementation-plan-splitter" "project-mgmt/implementation-plan-splitter"
migrate_audit "create-context-file" "project-mgmt/create-context-file"

# Specialized domain
migrate_audit "colyseus-multiplayer" "specialized/colyseus-multiplayer"
migrate_audit "github-copilot-models" "specialized/github-copilot-models"
migrate_audit "gitlab-api" "specialized/gitlab-api"

echo ""
echo "✅ Audit path migration complete"
