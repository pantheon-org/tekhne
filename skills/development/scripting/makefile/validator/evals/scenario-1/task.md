# Security Audit of a Deployment Makefile

## Problem/Feature Description

A DevOps team has been using a Makefile to drive deployments, database migrations, and cleanup operations across their staging environment. A junior engineer recently committed a version of the Makefile to the company's public GitHub repository before realizing it had secrets in it. The repository was made private again and the secrets rotated, but the team's security lead wants a proper audit of the current Makefile to identify any remaining security issues, unsafe patterns, and what safeguards need to be added before it can be used safely.

Review the Makefile below, identify all security concerns and unsafe patterns, produce a hardened version, and document each issue with an explanation of the risk.

## Output Specification

Produce:
- A corrected `Makefile` with security issues fixed
- A `security-audit.md` documenting each issue found, the risk it poses, and the fix applied

## Input Files

The following file is provided as input. Extract it before beginning.

=============== FILE: inputs/Makefile ===============
DB_HOST = prod-db.internal
DB_USER = admin
DB_PASS = hunter2
API_KEY = sk-live-abc123def456
DEPLOY_DIR = /opt/app

.EXPORT_ALL_VARIABLES:

.PHONY: deploy migrate clean rollback

deploy:
	rsync -av . $(DEPLOY_DIR)
	systemctl restart app

migrate:
	psql -h $(DB_HOST) -U $(DB_USER) -p $(DB_PASS) -c "SELECT 1"

clean:
	rm -rf $(DEPLOY_DIR)
	rm -rf $(BUILD_DIR)

rollback:
	sudo rm -rf $(DEPLOY_DIR)
	make deploy
