export class RedmineError extends Error {
  constructor(message, details = {}) {
    super(message);
    this.name = "RedmineError";
    this.status = details.status;
    this.method = details.method;
    this.path = details.path;
    this.details = details.details;
  }
}

export function parseBoolean(value, fallback = false) {
  if (value === undefined || value === null || value === "") {
    return fallback;
  }
  if (typeof value === "boolean") {
    return value;
  }
  return ["1", "true", "yes", "on"].includes(String(value).trim().toLowerCase());
}

export function createConfig(env = process.env, fetchImpl = globalThis.fetch) {
  return {
    baseUrl: trimTrailingSlash(env.REDMINE_BASE_URL || ""),
    apiKey: env.REDMINE_API_KEY || "",
    readOnly: parseBoolean(env.REDMINE_MCP_READ_ONLY, false),
    disabledFeatures: {
      attachments: parseBoolean(env.REDMINE_MCP_DISABLE_ATTACHMENTS, false),
      checklists: parseBoolean(env.REDMINE_MCP_DISABLE_CHECKLISTS, false),
      relations: parseBoolean(env.REDMINE_MCP_DISABLE_RELATIONS, false),
      timeEntries: parseBoolean(env.REDMINE_MCP_DISABLE_TIME_ENTRIES, false),
      versions: parseBoolean(env.REDMINE_MCP_DISABLE_VERSIONS, false),
      wiki: parseBoolean(env.REDMINE_MCP_DISABLE_WIKI, false),
      watchers: parseBoolean(env.REDMINE_MCP_DISABLE_WATCHERS, false),
    },
    silentWrites: parseBoolean(env.REDMINE_SILENT_WRITES, false),
    enableDeletes: parseBoolean(env.REDMINE_MCP_ENABLE_DELETES, false),
    attachmentMaxBytes: positiveNumber(env.REDMINE_MCP_ATTACHMENT_MAX_BYTES, 10485760),
    timeoutMs: positiveNumber(env.REDMINE_TIMEOUT_MS, 30000),
    fetchImpl,
  };
}

export function createRedmineClient(config = createConfig()) {
  return new RedmineClient(config);
}

export class RedmineClient {
  constructor(config) {
    this.config = {
      ...config,
      baseUrl: trimTrailingSlash(config.baseUrl || ""),
      disabledFeatures: {
        attachments: false,
        checklists: false,
        relations: false,
        timeEntries: false,
        versions: false,
        wiki: false,
        watchers: false,
        ...(config.disabledFeatures || {}),
      },
      timeoutMs: positiveNumber(config.timeoutMs, 30000),
      enableDeletes: parseBoolean(config.enableDeletes, false),
    };
    if (typeof this.config.fetchImpl !== "function") {
      throw new Error("Redmine MCP server requires a fetch implementation");
    }
  }

  async request(method, path, options = {}) {
    this.assertConfigured();

    const url = buildUrl(this.config.baseUrl, path, options.query);
    const headers = {
      "X-Redmine-API-Key": this.config.apiKey,
      Accept: options.accept || "application/json",
    };

    const request = {
      method,
      headers,
    };

    if (options.rawBody !== undefined) {
      headers["Content-Type"] = options.contentType || "application/octet-stream";
      request.body = options.rawBody;
    } else if (options.body !== undefined) {
      headers["Content-Type"] = "application/json";
      request.body = JSON.stringify(options.body);
    }

    const controller = new AbortController();
    const timeout =
      this.config.timeoutMs > 0
        ? setTimeout(() => controller.abort(), this.config.timeoutMs)
        : null;
    request.signal = controller.signal;

    let response;
    try {
      response = await this.config.fetchImpl(url, request);
    } catch (error) {
      const aborted = error && error.name === "AbortError";
      throw new RedmineError(
        aborted
          ? `Redmine request timed out after ${this.config.timeoutMs}ms`
          : `Redmine request failed: ${error.message || error}`,
        { method, path }
      );
    } finally {
      if (timeout) {
        clearTimeout(timeout);
      }
    }

    const payload = await parseResponseBody(response, options.responseType, response.ok);
    if (!response.ok) {
      throw new RedmineError(
        `Redmine ${method} ${path} returned HTTP ${response.status}`,
        {
          status: response.status,
          method,
          path,
          details: payload,
        }
      );
    }

    return {
      status: response.status,
      headers: response.headers,
      body: payload,
    };
  }

  assertConfigured() {
    if (!this.config.baseUrl) {
      throw new RedmineError("REDMINE_BASE_URL is not configured");
    }
    if (!this.config.apiKey) {
      throw new RedmineError("REDMINE_API_KEY is not configured");
    }
  }
}

export function buildUrl(baseUrl, path, query = {}) {
  const normalizedPath = path.startsWith("/") ? path.slice(1) : path;
  const url = new URL(`${trimTrailingSlash(baseUrl)}/${normalizedPath}`);

  for (const [key, value] of Object.entries(query || {})) {
    if (value === undefined || value === null || value === "") {
      continue;
    }
    url.searchParams.set(key, Array.isArray(value) ? value.join(",") : String(value));
  }

  return url.toString();
}

async function parseResponseBody(response, responseType, responseOk) {
  if (response.status === 204) {
    return null;
  }

  if (responseType === "buffer" && responseOk) {
    return Buffer.from(await response.arrayBuffer());
  }

  const text = await response.text();
  if (!text) {
    return null;
  }

  const contentType = response.headers.get("content-type") || "";
  if (contentType.includes("application/json")) {
    try {
      return JSON.parse(text);
    } catch {
      return text;
    }
  }

  try {
    return JSON.parse(text);
  } catch {
    return text;
  }
}

function trimTrailingSlash(value) {
  return String(value).replace(/\/+$/, "");
}

function positiveNumber(value, fallback) {
  const parsed = Number(value);
  return Number.isFinite(parsed) && parsed > 0 ? parsed : fallback;
}
