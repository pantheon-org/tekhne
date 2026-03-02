#!/usr/bin/env bun

const SKILL_PATH = process.argv[2] || "./test-skill/SKILL.md";
const API_BASE = "https://api.tessl.io";
const ENDPOINT = "/experimental/skills/review";

async function main() {
  const skillContent = await Bun.file(SKILL_PATH).text();

  console.log("Making request to:", API_BASE + ENDPOINT);
  console.log("Skill path:", SKILL_PATH);
  console.log("Content length:", skillContent.length, "chars");

  const response = await fetch(API_BASE + ENDPOINT, {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify({
      content: skillContent,
    }),
  });

  console.log("\nResponse status:", response.status);
  console.log("Response statusText:", response.statusText);

  const data = await response.json();
  console.log("\nResponse body:");
  console.log(JSON.stringify(data, null, 2));
}

main().catch(console.error);
