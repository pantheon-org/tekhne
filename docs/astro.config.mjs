// @ts-check

import starlight from "@astrojs/starlight";
import { defineConfig } from "astro/config";

// https://astro.build/config
export default defineConfig({
  site: "https://pantheon-org.github.io",
  base: "/tekhne",
  integrations: [
    starlight({
      title: "Tekhne Skills",
      description:
        "A curated collection of reusable agent skills for AI assistants.",
      logo: {
        src: "./src/assets/houston.webp",
        alt: "Tekhne",
      },
      social: [
        {
          icon: "github",
          label: "GitHub",
          href: "https://github.com/pantheon-org/tekhne",
        },
      ],
      editLink: {
        baseUrl: "https://github.com/pantheon-org/tekhne/edit/main/",
      },
      customCss: ["./src/styles/custom.css"],
      sidebar: [
        {
          label: "Overview",
          items: [{ label: "All Skills", slug: "tiles" }],
        },
        {
          label: "Agentic Harness",
          autogenerate: { directory: "skills/agentic-harness" },
          collapsed: true,
        },
        {
          label: "CI / CD",
          autogenerate: { directory: "skills/ci-cd" },
          collapsed: true,
        },
        {
          label: "Development",
          autogenerate: { directory: "skills/development" },
          collapsed: true,
        },
        {
          label: "Documentation",
          autogenerate: { directory: "skills/documentation" },
          collapsed: true,
        },
        {
          label: "Infrastructure",
          autogenerate: { directory: "skills/infrastructure" },
          collapsed: true,
        },
        {
          label: "Observability",
          autogenerate: { directory: "skills/observability" },
          collapsed: true,
        },
        {
          label: "Package Management",
          autogenerate: { directory: "skills/package-mgmt" },
          collapsed: true,
        },
        {
          label: "Project Management",
          autogenerate: { directory: "skills/project-mgmt" },
          collapsed: true,
        },
        {
          label: "Repository Management",
          autogenerate: { directory: "skills/repository-mgmt" },
          collapsed: true,
        },
        {
          label: "Software Engineering",
          autogenerate: { directory: "skills/software-engineering" },
          collapsed: true,
        },
        {
          label: "Specialized",
          autogenerate: { directory: "skills/specialized" },
          collapsed: true,
        },
        {
          label: "Testing",
          autogenerate: { directory: "skills/testing" },
          collapsed: true,
        },
      ],
    }),
  ],
});
