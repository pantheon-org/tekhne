export interface DomainInfo {
  key: string;
  title: string;
  description: string;
}

export const DOMAINS: DomainInfo[] = [
  {
    key: "ci-cd",
    title: "CI/CD",
    description: "CI/CD pipelines & deployment automation",
  },
  {
    key: "infrastructure",
    title: "Infrastructure",
    description: "Infrastructure as Code",
  },
  {
    key: "repository-mgmt",
    title: "Repository Management",
    description: "Repository & workspace management",
  },
  {
    key: "development",
    title: "Development",
    description: "Development tooling",
  },
  {
    key: "agentic-harness",
    title: "Agentic Harness",
    description: "Agent framework configurations",
  },
  {
    key: "testing",
    title: "Testing",
    description: "Testing methodologies & quality",
  },
  {
    key: "software-engineering",
    title: "Software Engineering",
    description: "Software engineering principles",
  },
  {
    key: "observability",
    title: "Observability",
    description: "Monitoring, logging & debugging",
  },
  {
    key: "documentation",
    title: "Documentation",
    description: "Writing & communication",
  },
  {
    key: "package-mgmt",
    title: "Package Management",
    description: "Package & version management",
  },
  {
    key: "project-mgmt",
    title: "Project Management",
    description: "Planning & organization",
  },
  {
    key: "specialized",
    title: "Specialized",
    description: "Domain-specific tools",
  },
  {
    key: "languages",
    title: "Languages",
    description: "Language-specific guidance and patterns",
  },
];

export const getDomainInfo = (domainKey: string): DomainInfo | undefined => {
  return DOMAINS.find((d) => d.key === domainKey);
};
