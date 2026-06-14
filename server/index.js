#!/usr/bin/env node

import { readFileSync } from "node:fs";
import readline from "node:readline";
import { createConfig, createRedmineClient } from "./src/redmine.js";
import { InputError, callTool, listTools, toolErrorPayload } from "./src/tools.js";

const protocolVersion = "2024-11-05";
const packageJson = JSON.parse(readFileSync(new URL("../package.json", import.meta.url), "utf8"));
const serverVersion = packageJson.version;
const client = createRedmineClient(createConfig());

const rl = readline.createInterface({
  input: process.stdin,
  crlfDelay: Infinity,
});

rl.on("line", async (line) => {
  const trimmed = line.trim();
  if (!trimmed) {
    return;
  }

  let message;
  try {
    message = JSON.parse(trimmed);
  } catch (error) {
    writeError(null, -32700, `Parse error: ${error.message}`);
    return;
  }

  try {
    const result = await handleMessage(message);
    if (message.id !== undefined && message.id !== null) {
      writeResult(message.id, result);
    }
  } catch (error) {
    if (message.id !== undefined && message.id !== null) {
      const code = error instanceof InputError ? -32602 : -32603;
      writeError(message.id, code, error.message || String(error));
    } else {
      console.error(error.message || error);
    }
  }
});

async function handleMessage(message) {
  switch (message.method) {
    case "initialize":
      return {
        protocolVersion: message.params?.protocolVersion || protocolVersion,
        capabilities: {
          tools: {
            listChanged: false,
          },
        },
        serverInfo: {
          name: "redmine",
          version: serverVersion,
        },
      };

    case "tools/list":
      return {
        tools: listTools(client.config),
      };

    case "tools/call":
      return handleToolCall(message.params || {});

    case "resources/list":
      return {
        resources: [],
      };

    case "prompts/list":
      return {
        prompts: [],
      };

    case "ping":
      return {};

    case "notifications/initialized":
      return undefined;

    default:
      throw new InputError(`Unsupported method: ${message.method}`);
  }
}

async function handleToolCall(params) {
  const name = params.name;
  const args = params.arguments || {};
  if (!name) {
    throw new InputError("tools/call requires params.name");
  }

  try {
    const result = await callTool(client, name, args);
    return contentResult(result);
  } catch (error) {
    if (error instanceof InputError) {
      throw error;
    }
    return contentResult(toolErrorPayload(error), true);
  }
}

function contentResult(payload, isError = false) {
  return {
    content: [
      {
        type: "text",
        text: JSON.stringify(payload, null, 2),
      },
    ],
    isError,
  };
}

function writeResult(id, result) {
  process.stdout.write(
    `${JSON.stringify({
      jsonrpc: "2.0",
      id,
      result,
    })}\n`
  );
}

function writeError(id, code, message) {
  process.stdout.write(
    `${JSON.stringify({
      jsonrpc: "2.0",
      id,
      error: {
        code,
        message,
      },
    })}\n`
  );
}
