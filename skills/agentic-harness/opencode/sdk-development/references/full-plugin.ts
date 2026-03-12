/**
 * Full OpenCode Plugin Example
 *
 * This file demonstrates a complete plugin with:
 * - Custom tools
 * - Event handling
 * - Chat hooks
 * - Permission handling
 * - Tool execution hooks
 *
 * Plugins provide more comprehensive integrations than standalone tools.
 */

import type { Hooks, Plugin } from "@opencode-ai/plugin";
import { tool } from "@opencode-ai/plugin";

type PluginInput = Parameters<Plugin>[0];
type PluginStats = {
  eventsReceived: number;
  messagesProcessed: number;
  toolsExecuted: number;
};

const buildProjectInfo = async (
  client: PluginInput["client"],
  worktree: string,
  projectName: string,
  includeFiles: boolean,
): Promise<string> => {
  let info = `Project: ${projectName}\nPath: ${worktree}\n`;

  if (!includeFiles) {
    return info;
  }

  const { data } = await client.file.list({
    query: { path: worktree, recursive: false },
  });
  if (data) {
    type ListedFile = { name?: string };
    info += `\nFiles:\n${data.map((file) => `  ${(file as ListedFile).name ?? "<unknown>"}`).join("\n")}`;
  }

  return info;
};

const createToolHooks = (
  input: PluginInput,
  stats: PluginStats,
): NonNullable<Hooks["tool"]> => {
  const { client, project, worktree, $ } = input;

  return {
    projectInfo: tool({
      description: "Get information about the current project",
      args: {
        includeFiles: tool.schema
          .boolean()
          .default(false)
          .describe("Include file listing"),
      },
      async execute({ includeFiles }) {
        return buildProjectInfo(client, worktree, project.name, includeFiles);
      },
    }),
    pluginStats: tool({
      description: "Get statistics about plugin activity",
      args: {},
      async execute() {
        return JSON.stringify(stats, null, 2);
      },
    }),
    gitStatus: tool({
      description: "Get the current git status",
      args: {
        short: tool.schema.boolean().default(true).describe("Use short format"),
      },
      async execute({ short }) {
        try {
          const flag = short ? "-s" : "";
          const result = await $`cd ${worktree} && git status ${flag}`.text();
          return result || "Working tree is clean";
        } catch (error) {
          return `Error: ${error instanceof Error ? error.message : String(error)}`;
        }
      },
    }),
  };
};

const createHooks = (input: PluginInput, stats: PluginStats): Hooks => {
  const { project } = input;

  return {
    tool: createToolHooks(input, stats),
    event: async ({ event }) => {
      stats.eventsReceived++;
      switch (event.type) {
        case "session.created":
          console.log(`[Plugin] New session: ${event.data?.id}`);
          break;
        case "message.created":
          console.log(
            `[Plugin] New message in session: ${event.data?.sessionID}`,
          );
          break;
        case "tool.completed":
          console.log(`[Plugin] Tool completed: ${event.data?.tool}`);
          stats.toolsExecuted++;
          break;
      }
    },
    config: async (config) => {
      console.log(`[Plugin] Config loaded, current theme: ${config.theme}`);
    },
    "chat.message": async (input, _output) => {
      stats.messagesProcessed++;
      const _contextPart = {
        type: "text" as const,
        text: `\n[Context: Project "${project.name}" | Session ${input.sessionID}]`,
      };
      console.log(`[Plugin] Processing message for agent: ${input.agent}`);
    },
    "chat.params": async (input, output) => {
      if (input.agent === "code") {
        output.temperature = 0.3;
      } else if (input.agent === "creative") {
        output.temperature = 0.9;
      }
      console.log(
        `[Plugin] Params for ${input.agent}: temp=${output.temperature}`,
      );
    },
    "permission.ask": async (input, output) => {
      const safeTools = ["Read", "Glob", "Grep"];
      if (safeTools.includes(input.tool || "")) {
        output.status = "allow";
        console.log(`[Plugin] Auto-allowed safe tool: ${input.tool}`);
      } else {
        output.status = "ask";
      }
    },
    "tool.execute.before": async (input, output) => {
      console.log(`[Plugin] Tool starting: ${input.tool}`);
      if (typeof output.args === "object" && output.args !== null) {
        (output.args as Record<string, unknown>)._pluginTimestamp = Date.now();
      }
    },
    "tool.execute.after": async (input, output) => {
      const duration =
        typeof output.metadata?._pluginTimestamp === "number"
          ? Date.now() - output.metadata._pluginTimestamp
          : 0;
      console.log(`[Plugin] Tool completed: ${input.tool} (${duration}ms)`);
      output.title = `${output.title} [via plugin]`;
      stats.toolsExecuted++;
    },
  };
};

/**
 * Plugin entry point
 * Receives context about the current project and OpenCode client
 */
const myPlugin: Plugin = async (input) => {
  const { project, directory, worktree } = input;

  // Log plugin initialization
  console.log(`Plugin loaded for project: ${project.name}`);
  console.log(`Directory: ${directory}`);
  console.log(`Worktree: ${worktree}`);

  // Track statistics for this session
  const stats = {
    eventsReceived: 0,
    messagesProcessed: 0,
    toolsExecuted: 0,
  };

  return createHooks(input, stats);
};

export default myPlugin;

/**
 * Alternative: Minimal plugin with just tools
 */
export const minimalPlugin: Plugin = async ({ project }) => ({
  tool: {
    hello: tool({
      description: "Say hello from the plugin",
      args: {},
      async execute() {
        return `Hello from ${project.name}!`;
      },
    }),
  },
});

/**
 * Alternative: Plugin with custom auth provider
 */
export const authPlugin: Plugin = async () => ({
  auth: {
    provider: "my-api",
    methods: [
      {
        type: "api",
        label: "API Key",
        prompts: [
          {
            type: "text",
            key: "apiKey",
            message: "Enter your API key",
            placeholder: "key_...",
            validate: (value) => {
              if (!value.startsWith("key_")) {
                return "API key must start with 'key_'";
              }
              if (value.length < 10) {
                return "API key is too short";
              }
              return undefined; // Valid
            },
          },
        ],
        authorize: async (inputs) => {
          if (!inputs?.apiKey) {
            return { type: "failed" };
          }

          // Validate the key (example)
          const isValid = inputs.apiKey.startsWith("key_");

          if (isValid) {
            return {
              type: "success",
              key: inputs.apiKey,
            };
          }

          return { type: "failed" };
        },
      },
    ],
  },
});
