# Scenario 01: Security Audit of a Deployment Makefile

## User Prompt

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

## Expected Behavior

1. Remove the literal `hunter2` password — `DB_PASS` uses an environment variable reference or is removed
2. Remove the literal `sk-live-abc123def456` API key — `API_KEY` uses an environment variable reference or is removed
3. Remove the `.EXPORT_ALL_VARIABLES:` directive from the corrected Makefile
4. Explain in `security-audit.md` that `.EXPORT_ALL_VARIABLES` exports all variables (including secrets) to every subprocess
5. Update `rm -rf $(DEPLOY_DIR)` and/or `rm -rf $(BUILD_DIR)` to quote the variables in the corrected Makefile
6. Replace bare `make deploy` in the rollback target with `$(MAKE) deploy`
7. Explain in `security-audit.md` the risk of hardcoded credentials (committed to source control, visible in config files)
8. Explain in `security-audit.md` why `$(MAKE)` should be used instead of bare `make` for recursive calls

## Success Criteria

- **Hardcoded password removed**: The corrected Makefile does NOT contain the literal value `hunter2`
- **Hardcoded API key removed**: The corrected Makefile does NOT contain the literal value `sk-live-abc123def456`
- **.EXPORT_ALL_VARIABLES removed**: The `.EXPORT_ALL_VARIABLES:` directive is removed from the corrected Makefile
- **EXPORT_ALL risk explained**: `security-audit.md` explains that `.EXPORT_ALL_VARIABLES` exports all variables (including secrets) to every subprocess
- **Unquoted rm fixed**: The `rm -rf $(DEPLOY_DIR)` and/or `rm -rf $(BUILD_DIR)` commands are updated to quote the variable
- **Bare make replaced**: Bare `make deploy` in the rollback target is replaced with `$(MAKE) deploy`
- **Credentials risk explained**: `security-audit.md` explains the risk of hardcoded credentials
- **Bare make risk explained**: `security-audit.md` explains why `$(MAKE)` should be used instead of bare `make`

## Failure Conditions

- Agent leaves `hunter2` in the corrected Makefile
- Agent leaves `sk-live-abc123def456` in the corrected Makefile
- Agent leaves `.EXPORT_ALL_VARIABLES:` in the corrected Makefile
- Agent does not explain the `.EXPORT_ALL_VARIABLES` risk in `security-audit.md`
- Agent does not quote the `rm -rf` variable
- Agent leaves bare `make deploy` instead of `$(MAKE) deploy`
- Agent does not explain the credentials exposure risk in `security-audit.md`
