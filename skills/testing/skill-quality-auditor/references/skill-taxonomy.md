# Skill Taxonomy & Classification Guide

**Purpose:** Guide skill classification and domain organization for the Tekhne skill repository.

**Last Updated:** 2026-03-03  
**Domains:** 12 (ci-cd, infrastructure, repository-mgmt, development, agentic-harness, testing, software-engineering, observability, documentation, package-mgmt, project-mgmt, specialized)

---

## Domain Definitions

### ci-cd/
**Scope:** CI/CD pipelines, deployment automation, release management

**Includes:**
- GitHub Actions, GitLab CI, Azure Pipelines (pipeline platforms)
- Jenkins (CI/CD orchestration)
- Helm (Kubernetes package manager & deployment)
- FluentBit (log forwarding for observability pipelines)

**Excludes:**
- Infrastructure provisioning (use infrastructure/ instead)
- Monitoring/alerting tools (use observability/ instead)

**Structure:** Paired generator/validator skills nested under tool name
```
ci-cd/
├── github-actions/{generator,validator}
├── gitlab-ci/{generator,validator}
└── ...
```

---

### infrastructure/
**Scope:** Infrastructure as Code, cloud provisioning, container management

**Includes:**
- Terraform, Terragrunt (IaC provisioning)
- Ansible (configuration management)
- Docker (containerization)
- Kubernetes YAML (K8s resource definitions)
- CloudFormation tools (cfn/ subdomain)
- AWS CDK (aws-cdk/ subdomain)

**Excludes:**
- Deployment pipelines (use ci-cd/ instead)
- Monitoring tools (use observability/ instead)

**Subdomains:**
- `cfn/` - CloudFormation-specific tools (behavior-validator, template-compare)
- `aws-cdk/` - AWS CDK ecosystem (cdk-nag, future: cdk-patterns, cdk-constructs)

**Structure:** Paired tools nested, standalone tools at subdomain level
```
infrastructure/
├── terraform/{generator,validator}
├── cfn/
│   ├── behavior-validator/
│   └── template-compare/
└── aws-cdk/
    └── cdk-nag/
```

---

### repository-mgmt/
**Scope:** Repository management, monorepo orchestration, git workflows, workspace organization

**Includes:**
- Nx workspace tools (nx/ subdomain)
- Future: Turborepo, Lerna, Rush, Moon
- Git workflows, branching strategies
- Workspace management patterns

**Excludes:**
- General development tools not specific to repo management (use development/ instead)
- CI/CD pipelines (use ci-cd/ instead)

**Subdomains:**
- `nx/` - Nx monorepo ecosystem

**Why repository-mgmt (not monorepo):** Broader scope includes general repository management, not exclusive to monorepos

**Structure:** Tools organized under ecosystem subdomain
```
repository-mgmt/
└── nx/
    ├── workspace-patterns/
    ├── executors/
    └── ...
```

---

### development/
**Scope:** General-purpose development tools, languages, runtimes, formatters, linters

**Includes:**
- Bun, TypeScript (languages/runtimes)
- Biome (formatter/linter)
- Commander.js (CLI framework)
- Shell scripting tools (scripting/ subdomain)

**Excludes:**
- Repository management (use repository-mgmt/ instead)
- Agent framework configs (use agentic-harness/ instead)
- Testing tools (use testing/ instead)

**Subdomains:**
- `scripting/` - Shell scripting and Makefile generation

**Structure:** Mix of standalone skills and subdomains
```
development/
├── bun-development/
├── typescript-advanced/
└── scripting/
    ├── bash-script/{generator,validator}
    └── makefile/{generator,validator}
```

---

### agentic-harness/
**Scope:** Agent framework configurations, agent-specific documentation, agent collaboration patterns

**Includes:**
- OpenCode configuration
- AGENTS.md documentation (agent collaboration)
- Future: Cursor, Claude Desktop, Windsurf, Aider configs

**Excludes:**
- General documentation (use documentation/ instead)
- Development tools not specific to agents (use development/ instead)

**Why Top-Level:** Agent frameworks and agent-specific tooling are a distinct category warranting their own domain for discoverability

**Structure:** Flat list of framework-specific skills
```
agentic-harness/
├── opencode/
├── agents-md/
└── [future: cursor, claude-desktop, etc.]
```

---

### testing/
**Scope:** Testing methodologies, test frameworks, quality assurance, debugging workflows

**Includes:**
- BDD testing (Behavior-Driven Development)
- TDD (Test-Driven Development)
- Skill quality auditor
- UI debugging workflows

**Excludes:**
- Software engineering principles (use software-engineering/ instead)
- Observability/monitoring (use observability/ instead)

**Structure:** Flat list of testing methodology skills
```
testing/
├── bdd-testing/
├── test-driven-development/
├── skill-quality-auditor/
└── ui-debug-workflow/
```

---

### software-engineering/
**Scope:** Software engineering principles, design patterns, architecture patterns, refactoring techniques, code quality fundamentals

**Includes:**
- Software design principles (SOLID, Gang of Four, Clean Architecture, Hexagonal Architecture)
- Design patterns
- Architecture patterns
- Refactoring techniques

**Excludes:**
- Testing methodologies (use testing/ instead)
- Development tools (use development/ instead)

**Why Top-Level:** Software engineering principles are foundational and cross-cutting; not specific to testing, development, or any other domain

**Future Expansion:** May split into multiple focused skills if comprehensive skill becomes too large (design-patterns, architecture-patterns, refactoring-techniques)

**Structure:** Flat list (currently single comprehensive skill)
```
software-engineering/
└── software-design-principles/
```

---

### observability/
**Scope:** Monitoring, logging, debugging, metrics, alerting

**Includes:**
- PromQL (Prometheus query language)
- LogQL (Loki query language)
- K8s debugging tools

**Excludes:**
- Testing frameworks (use testing/ instead)
- CI/CD pipelines (use ci-cd/ instead)

**Structure:** Mix of paired tools and standalone skills
```
observability/
├── promql/{generator,validator}
├── logql-generator/
└── k8s-debug/
```

---

### documentation/
**Scope:** General-purpose writing, technical communication, documentation practices

**Includes:**
- Markdown authoring (general markdown syntax)
- Acceptance criteria (user story requirements)
- Conventional commits (commit message standards)
- Plain English (technical writing for non-technical audiences)
- Journal entries (personal documentation)

**Excludes:**
- Agent-specific documentation (use agentic-harness/ instead)
- API documentation tools (may belong in development/)

**Why Not agents-md:** AGENTS.md is specifically for AI agent collaboration, not general technical documentation

**Structure:** Flat list of documentation skills
```
documentation/
├── markdown-authoring/
├── acceptance-criteria/
├── conventional-commits/
├── plain-english/
└── journal-entry-creator/
```

---

### package-mgmt/
**Scope:** Package management, version management, dependency management

**Includes:**
- Mise (cross-language version manager)
- Future: npm/yarn/pnpm, pip, cargo, gem, etc.

**Excludes:**
- Build tools (use development/ or repository-mgmt/ instead)
- Deployment tools (use ci-cd/ instead)

**Broad Scope:** Encompasses language-specific, cross-language, and Node ecosystem package management tools

**Structure:** Flat list of package management tools
```
package-mgmt/
└── mise-complete/
```

---

### project-mgmt/
**Scope:** Planning, prioritization, task management, project organization

**Includes:**
- Moscow prioritization (requirement prioritization)
- Implementation plan splitter (breaking down work)
- Context file creator (planning artifacts)

**Excludes:**
- Development workflows (use development/ or testing/ instead)
- Repository management (use repository-mgmt/ instead)

**Structure:** Flat list of planning/organization skills
```
project-mgmt/
├── moscow-prioritization/
├── implementation-plan-splitter/
└── create-context-file/
```

---

### specialized/
**Scope:** Domain-specific tools that don't fit other categories

**Includes:**
- Colyseus multiplayer (game server framework)
- GitHub Copilot models (AI model listings)
- GitLab API (platform-specific API)

**Use Sparingly:** Only when skill truly doesn't belong elsewhere

**Structure:** Flat list of specialized skills
```
specialized/
├── colyseus-multiplayer/
├── github-copilot-models/
└── gitlab-api/
```

---

## Classification Decision Tree

Use this flowchart to classify new skills:

1. **Is it about deploying/releasing code?** → ci-cd/
2. **Is it about provisioning infrastructure?** → infrastructure/
3. **Is it specific to repository/workspace management?** → repository-mgmt/
4. **Is it about agent frameworks or agent collaboration?** → agentic-harness/
5. **Is it about monitoring/logging production systems?** → observability/
6. **Is it about testing or debugging?** → testing/
7. **Is it about software engineering principles/patterns?** → software-engineering/
8. **Is it about general-purpose writing/documentation?** → documentation/
9. **Is it about package/version management?** → package-mgmt/
10. **Is it a general development tool?** → development/
11. **Is it about project planning/organization?** → project-mgmt/
12. **Doesn't fit anywhere else?** → specialized/ (use sparingly)

---

## Borderline Cases

### agents-md (AGENTS.md documentation)
**Placement:** agentic-harness/  
**Rationale:** AGENTS.md files are specifically for AI agent collaboration and orchestration  
**Alternative considered:** documentation/ (general documentation) - rejected because agent-specific, not general-purpose writing

### software-design-principles
**Placement:** software-engineering/ (new top-level domain)  
**Rationale:** Foundational engineering principles are cross-cutting and comprehensive  
**Alternative considered:** testing/ (quality tool) - rejected because not specific to testing, applies to all development

### mise-complete
**Placement:** package-mgmt/ (new top-level domain)  
**Rationale:** Cross-language version/package management tool  
**Alternative considered:** development/ (dev tool) - rejected because package management is a distinct domain with future growth potential

### extending-nx-plugins
**Placement:** repository-mgmt/nx/  
**Rationale:** Teaches extending Nx itself, which is specific to repository management  
**Alternative considered:** development/ (general tooling) - rejected because too Nx-specific

### cdk-nag
**Placement:** infrastructure/aws-cdk/  
**Rationale:** AWS CDK-specific validation tool, warrants CDK subdomain  
**Alternative considered:** infrastructure/cfn/ (CFN tools) - rejected because CDK is distinct from CloudFormation, even though it generates CFN

### cfn-behavior-validator, cfn-template-compare
**Placement:** infrastructure/cfn/  
**Rationale:** Grouped CloudFormation-specific tools under subdomain  
**Alternative considered:** infrastructure/ (flat) - rejected because grouping related CFN tools improves organization

### bash-script, makefile
**Placement:** development/scripting/  
**Rationale:** Shell scripting is a specialized development activity  
**Alternative considered:** top-level scripting/ domain - rejected because scripting is fundamentally a development tool, nesting under development/ keeps related tools together

### ui-debug-workflow
**Placement:** testing/  
**Rationale:** Testing methodology for UI changes  
**Alternative considered:** development/ (dev tool) - rejected because primarily about testing, not general development

---

## Adding New Skills

When adding a new skill:

1. **Read domain definitions** above to understand scope and boundaries
2. **Run through decision tree** to identify best-fit domain
3. **Check for existing related skills** in candidate domain for consistency
4. **Consider generator/validator pairing:**
   - If creating both, use nested structure: `domain/tool/{generator,validator}/`
   - If standalone, place directly in domain or subdomain: `domain/skill-name/`
5. **Consider subdomain grouping:**
   - If 3+ related skills exist, consider creating subdomain
   - Example: infrastructure/cfn/ groups CloudFormation tools
6. **Document borderline decisions** in this file for future reference
7. **Update this guide** if skill doesn't fit existing domains

---

## Creating New Domains

Before creating a new domain:

1. **Verify at least 3-5 skills** would belong in the new domain
2. **Ensure it's not a subset** of existing domains
3. **Define clear scope** that doesn't overlap with existing domains
4. **Consider future growth:** Will this domain attract more skills?
5. **Update this guide** with domain definition and decision tree
6. **Update AGENTS.md** with new domain information
7. **Get team consensus** before restructuring

**Recent Examples:**
- `software-engineering/` created for design principles (foundational, cross-cutting)
- `agentic-harness/` created for agent frameworks (distinct category, future growth)
- `package-mgmt/` created for version managers (broad scope, future growth)

---

## Subdomain Strategy

**When to create a subdomain:**
- 3+ related skills under a common tool/platform (e.g., nx/, cfn/, aws-cdk/)
- Clear separation from other skills in the domain
- Future growth expected in that subdomain

**Current subdomains:**
- `infrastructure/aws-cdk/` - AWS CDK ecosystem (allows cdk-patterns, cdk-constructs)
- `infrastructure/cfn/` - CloudFormation tools (grouped related functionality)
- `repository-mgmt/nx/` - Nx ecosystem (allows turborepo, lerna, rush)
- `development/scripting/` - Shell scripting tools (bash, makefile)

**Future subdomain candidates:**
- `ci-cd/github/` - If we add github-release, github-checks, etc.
- `development/nodejs/` - If we add multiple Node.js-specific tools
- `agentic-harness/cursor/` - If we add multiple Cursor-specific skills

---

## Generator/Validator Pairing

**When to nest generator/validator:**
- Both skills exist for the same tool
- They share common concepts and workflows
- Users typically need both for complete workflow

**Structure:**
```
domain/
└── tool/
    ├── generator/
    │   ├── SKILL.md
    │   └── tile.json
    └── validator/
        ├── SKILL.md
        └── tile.json
```

**Examples:**
- `ci-cd/github-actions/{generator,validator}`
- `infrastructure/terraform/{generator,validator}`
- `development/scripting/bash-script/{generator,validator}`

**When NOT to nest:**
- Only generator OR validator exists (place directly in domain)
- Generator and validator are conceptually different tools (place separately)

---

## Taxonomy Maintenance

### Quarterly Review Process
1. **Check domain balance** - Avoid over-concentration in single domain
2. **Evaluate subdomain candidates** - Are there 3+ related skills that should be grouped?
3. **Review borderline cases** - Are any skills misclassified based on new context?
4. **Update decision tree** - Add new questions based on recent classification challenges
5. **Document new patterns** - Add borderline cases as they arise

### Indicators for Domain Refinement
- **Domain has 20+ skills** - Consider splitting into subdomains or multiple domains
- **Subdomain has 1 skill for 6+ months** - Consider flattening into parent domain
- **Frequent classification debates** - Scope definition needs clarification
- **Cross-domain dependencies** - May indicate skills belong together

### When to Split a Domain
Example: If `infrastructure/` grows to 25+ skills, consider:
- Split by cloud provider: `infrastructure/aws/`, `infrastructure/gcp/`, `infrastructure/azure/`
- Split by tool type: `infrastructure/iac/` (Terraform, etc.), `infrastructure/containers/` (Docker, K8s)

### When to Merge Domains
Example: If `package-mgmt/` remains at 1-2 skills for 12+ months:
- Consider merging into `development/` domain
- Document why package management didn't need separate domain

---

## Common Classification Questions

### Q: Where do API client skills belong?
**A:** Depends on specificity:
- Platform-specific (GitLab API) → specialized/
- General HTTP/REST skills → development/
- Integration with existing domains (AWS API) → infrastructure/

### Q: Where do language-specific tools belong?
**A:**
- General-purpose language tools (TypeScript) → development/
- Language-specific testing frameworks → testing/
- Language package managers → package-mgmt/

### Q: Where do deployment tools belong?
**A:**
- CI/CD pipelines and automation → ci-cd/
- Infrastructure provisioning → infrastructure/
- Container orchestration → infrastructure/

### Q: Where do documentation generation tools belong?
**A:**
- General documentation → documentation/
- Agent-specific documentation → agentic-harness/
- API documentation (might belong in development/, case-by-case)

### Q: What if a skill spans multiple domains?
**A:**
- Choose primary domain based on main use case
- Document cross-domain nature in skill frontmatter
- Consider splitting into multiple focused skills

---

## Taxonomy Evolution History

### 2026-03-03 - Initial 12-Domain Structure
- Reorganized from flat 63-skill structure
- Created domains: ci-cd, infrastructure, repository-mgmt, development, agentic-harness, testing, software-engineering, observability, documentation, package-mgmt, project-mgmt, specialized
- Renamed monorepo → repository-mgmt (broader scope)
- Moved agents-md from documentation to agentic-harness
- Created subdomains: infrastructure/cfn/, infrastructure/aws-cdk/, repository-mgmt/nx/, development/scripting/

**Rationale:** Improve discoverability with 63 skills, logical grouping, room for future growth

---

## References

- [Agent Skills Specification](https://agentskills.io) - Cross-harness compatibility
- Repository: `/Users/thomas.roche/Projects/github/pantheon-org/tekhne`
- AGENTS.md: Repository collaboration guide with domain references

---

**Maintainers:** Update this guide when adding domains, reclassifying skills, or encountering new classification challenges.
