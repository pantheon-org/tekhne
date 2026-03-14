/**
 * Basic OpenCode Tool Example
 *
 * This file demonstrates how to create a simple custom tool for OpenCode.
 * Place this file in `.opencode/tool/` or `~/.config/opencode/tool/`
 *
 * The filename becomes the tool name (e.g., basic-tool)
 */

import { tool } from "@opencode-ai/plugin";

/**
 * Simple greeting tool
 * Demonstrates basic tool structure with string and optional parameters
 */
export default tool({
  description: "Generate a greeting message for a person",

  args: {
    // Required string parameter
    name: tool.schema.string().describe("The name of the person to greet"),

    // Optional string with default
    greeting: tool.schema
      .string()
      .default("Hello")
      .describe("The greeting word to use"),

    // Optional boolean
    formal: tool.schema
      .boolean()
      .default(false)
      .describe("Whether to use formal greeting"),
  },

  async execute({ name, greeting, formal }, context) {
    // Access context information
    console.log(`Session: ${context.sessionID}`);
    console.log(`Agent: ${context.agent}`);

    // Build the greeting
    if (formal) {
      return `${greeting}, ${name}. It is a pleasure to meet you.`;
    }

    return `${greeting}, ${name}!`;
  },
});

/**
 * File search tool
 * Demonstrates shell command execution
 */
export const searchFiles = tool({
  description: "Search for files matching a glob pattern",

  args: {
    pattern: tool.schema
      .string()
      .describe("Glob pattern to match (e.g., '*.ts')"),
    directory: tool.schema
      .string()
      .default(".")
      .describe("Directory to search in"),
    maxDepth: tool.schema
      .number()
      .int()
      .min(1)
      .max(10)
      .default(3)
      .describe("Maximum directory depth to search"),
  },

  async execute({ pattern, directory, maxDepth }) {
    // Using Bun's shell
    const { $ } = await import("bun");

    try {
      const result =
        await $`find ${directory} -maxdepth ${maxDepth} -name "${pattern}" -type f`.text();
      const files = result.trim().split("\n").filter(Boolean);

      if (files.length === 0) {
        return `No files found matching pattern "${pattern}" in ${directory}`;
      }

      return `Found ${files.length} file(s):\n${files.join("\n")}`;
    } catch (error) {
      return `Error searching files: ${error instanceof Error ? error.message : String(error)}`;
    }
  },
});

/**
 * JSON formatter tool
 * Demonstrates structured data handling
 */
export const formatJson = tool({
  description: "Format and validate JSON data",

  args: {
    json: tool.schema.string().describe("JSON string to format"),
    indent: tool.schema
      .number()
      .int()
      .min(0)
      .max(8)
      .default(2)
      .describe("Number of spaces for indentation"),
  },

  async execute({ json, indent }) {
    try {
      const parsed = JSON.parse(json);
      return JSON.stringify(parsed, null, indent);
    } catch (error) {
      return `Invalid JSON: ${error instanceof Error ? error.message : String(error)}`;
    }
  },
});

/**
 * Context info tool
 * Demonstrates accessing the full tool context
 */
export const contextInfo = tool({
  description: "Get information about the current execution context",

  args: {},

  async execute(_args, context) {
    return JSON.stringify(
      {
        sessionID: context.sessionID,
        messageID: context.messageID,
        agent: context.agent,
        aborted: context.abort.aborted,
      },
      null,
      2,
    );
  },
});

/**
 * HTTP fetch tool
 * Demonstrates async operations with abort handling
 */
export const fetchUrl = tool({
  description: "Fetch content from a URL",

  args: {
    url: tool.schema.string().url().describe("URL to fetch"),
    method: tool.schema
      .enum(["GET", "POST", "PUT", "DELETE"])
      .default("GET")
      .describe("HTTP method"),
    timeout: tool.schema
      .number()
      .int()
      .min(1000)
      .max(30000)
      .default(5000)
      .describe("Request timeout in milliseconds"),
  },

  async execute({ url, method, timeout }, context) {
    try {
      const controller = new AbortController();

      // Connect abort signals
      context.abort.addEventListener("abort", () => controller.abort());

      const timeoutId = setTimeout(() => controller.abort(), timeout);

      const response = await fetch(url, {
        method,
        signal: controller.signal,
      });

      clearTimeout(timeoutId);

      const contentType = response.headers.get("content-type") || "";
      const body = contentType.includes("application/json")
        ? JSON.stringify(await response.json(), null, 2)
        : await response.text();

      return `Status: ${response.status} ${response.statusText}\n\n${body.slice(0, 5000)}`;
    } catch (error) {
      if (error instanceof Error && error.name === "AbortError") {
        return "Request was aborted or timed out";
      }
      return `Fetch error: ${error instanceof Error ? error.message : String(error)}`;
    }
  },
});
