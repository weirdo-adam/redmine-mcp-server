import { RedmineError, parseBoolean } from "./redmine.js";

const readLimit = {
  type: "integer",
  minimum: 1,
  maximum: 100,
  default: 25,
};

const silentWriteProperties = {
  silent: {
    type: "boolean",
    description:
      "Override REDMINE_SILENT_WRITES for this call. When true, return compact output and pass notify=false.",
  },
  notify: {
    type: "boolean",
    description: "Override the Redmine notify query parameter for this write request.",
  },
};

const WRITE_TOOLS = new Set([
  "redmine_create_issue",
  "redmine_update_issue",
  "redmine_delete_issue",
  "redmine_upload_attachment",
  "redmine_delete_attachment",
  "redmine_add_issue_relation",
  "redmine_delete_issue_relation",
  "redmine_add_checklist_item",
  "redmine_update_checklist_item",
  "redmine_delete_checklist_item",
  "redmine_add_time_entry",
  "redmine_update_time_entry",
  "redmine_delete_time_entry",
  "redmine_create_version",
  "redmine_update_version",
  "redmine_delete_version",
  "redmine_add_watcher",
  "redmine_remove_watcher",
]);

const DELETE_TOOLS = new Set([
  "redmine_delete_issue",
  "redmine_delete_attachment",
  "redmine_delete_issue_relation",
  "redmine_delete_checklist_item",
  "redmine_delete_time_entry",
  "redmine_delete_version",
  "redmine_remove_watcher",
]);

const FEATURE_TOOLS = {
  attachments: new Set([
    "redmine_get_attachment",
    "redmine_download_attachment",
    "redmine_upload_attachment",
    "redmine_delete_attachment",
  ]),
  checklists: new Set([
    "redmine_list_checklists",
    "redmine_add_checklist_item",
    "redmine_update_checklist_item",
    "redmine_delete_checklist_item",
  ]),
  relations: new Set([
    "redmine_list_issue_relations",
    "redmine_get_issue_relation",
    "redmine_add_issue_relation",
    "redmine_delete_issue_relation",
  ]),
  timeEntries: new Set([
    "redmine_list_time_entries",
    "redmine_get_time_entry",
    "redmine_add_time_entry",
    "redmine_update_time_entry",
    "redmine_delete_time_entry",
    "redmine_list_time_entry_activities",
  ]),
  versions: new Set([
    "redmine_list_versions",
    "redmine_get_version",
    "redmine_create_version",
    "redmine_update_version",
    "redmine_delete_version",
  ]),
  wiki: new Set(["redmine_list_wiki_pages", "redmine_get_wiki_page"]),
  watchers: new Set([
    "redmine_list_watchers",
    "redmine_add_watcher",
    "redmine_remove_watcher",
  ]),
};

const FEATURE_ENV = {
  attachments: "REDMINE_MCP_DISABLE_ATTACHMENTS",
  checklists: "REDMINE_MCP_DISABLE_CHECKLISTS",
  relations: "REDMINE_MCP_DISABLE_RELATIONS",
  timeEntries: "REDMINE_MCP_DISABLE_TIME_ENTRIES",
  versions: "REDMINE_MCP_DISABLE_VERSIONS",
  wiki: "REDMINE_MCP_DISABLE_WIKI",
  watchers: "REDMINE_MCP_DISABLE_WATCHERS",
};

const DEFAULT_ISSUE_INCLUDES = ["journals", "watchers", "checklists", "relations"];
const DEFAULT_ATTACHMENT_MAX_BYTES = 10485760;
const WIKI_INCLUDES = ["attachments"];
const PROJECT_INCLUDES = [
  "trackers",
  "issue_categories",
  "enabled_modules",
  "time_entry_activities",
  "issue_custom_fields",
];

export class InputError extends Error {
  constructor(message) {
    super(message);
    this.name = "InputError";
  }
}

export const TOOLS = [
  {
    name: "redmine_get_issue",
    description:
      "Get one Redmine issue. Supports native includes for journals, watchers, checklists, relations, attachments, children, changesets, and allowed_statuses.",
    inputSchema: objectSchema(
      {
        issue_id: integer("Redmine issue ID."),
        include: {
          type: "array",
          items: {
            type: "string",
            enum: [
              "journals",
              "watchers",
              "checklists",
              "relations",
              "attachments",
              "children",
              "changesets",
              "allowed_statuses",
            ],
          },
          default: DEFAULT_ISSUE_INCLUDES,
        },
      },
      ["issue_id"]
    ),
  },
  {
    name: "redmine_list_issues",
    description: "List Redmine issues with common Redmine REST filters.",
    inputSchema: objectSchema({
      project_id: stringOrInteger("Project identifier or numeric ID."),
      status_id: stringOrInteger("Status filter such as open, closed, *, or a numeric status ID."),
      tracker_id: integer("Tracker ID."),
      assigned_to_id: stringOrInteger("Assignee ID, me, or none."),
      fixed_version_id: stringOrInteger("Version ID, none, or *."),
      query_id: integer("Saved query ID."),
      sort: { type: "string" },
      offset: { type: "integer", minimum: 0 },
      limit: readLimit,
    }),
  },
  {
    name: "redmine_create_issue",
    description: "Create a Redmine issue. Supports Redmine notify=false email suppression.",
    inputSchema: objectSchema(
      {
        project_id: stringOrInteger("Project identifier or numeric ID."),
        subject: { type: "string", description: "Issue subject." },
        description: { type: "string" },
        tracker_id: integer("Tracker ID."),
        status_id: integer("Status ID."),
        priority_id: integer("Priority ID."),
        assigned_to_id: integer("Assignee user ID."),
        category_id: integer("Issue category ID."),
        fixed_version_id: integer("Target version ID."),
        parent_issue_id: integer("Parent issue ID."),
        start_date: { type: "string", description: "Start date in YYYY-MM-DD format." },
        due_date: { type: "string", description: "Due date in YYYY-MM-DD format." },
        done_ratio: integer("Done ratio percentage."),
        estimated_hours: { type: "number", minimum: 0 },
        custom_fields: {
          type: "array",
          description: "Redmine custom_fields payload.",
          items: { type: "object", additionalProperties: true },
        },
        watcher_user_ids: {
          type: "array",
          description: "Watcher user IDs to add on creation.",
          items: { type: "integer" },
        },
        fields: {
          type: "object",
          description:
            "Additional Redmine issue payload fields. Explicit top-level fields override matching values.",
          additionalProperties: true,
        },
        ...silentWriteProperties,
      },
      ["project_id", "subject"]
    ),
  },
  {
    name: "redmine_update_issue",
    description:
      "Update a Redmine issue, including notes and checklists_attributes for the Redmine Checklists plugin.",
    inputSchema: objectSchema(
      {
        issue_id: integer("Redmine issue ID."),
        fields: {
          type: "object",
          description:
            "Issue payload fields, for example status_id, assigned_to_id, notes, custom_fields, or checklists_attributes.",
          additionalProperties: true,
        },
        ...silentWriteProperties,
      },
      ["issue_id", "fields"]
    ),
  },
  {
    name: "redmine_delete_issue",
    description: "Delete a Redmine issue. This is destructive and disabled in read-only mode.",
    inputSchema: objectSchema(
      {
        issue_id: integer("Redmine issue ID."),
        ...silentWriteProperties,
      },
      ["issue_id"]
    ),
  },
  {
    name: "redmine_search",
    description: "Search across Redmine using the native search API.",
    inputSchema: objectSchema(
      {
        q: { type: "string", description: "Search query." },
        scope: {
          type: "string",
          description: "Optional project scope, for example all or a project identifier.",
        },
        all_words: { type: "boolean" },
        titles_only: { type: "boolean" },
        issues: { type: "boolean" },
        news: { type: "boolean" },
        documents: { type: "boolean" },
        changesets: { type: "boolean" },
        wiki_pages: { type: "boolean" },
        messages: { type: "boolean" },
        projects: { type: "boolean" },
        open_issues: { type: "boolean" },
        attachments: { type: "boolean" },
        offset: { type: "integer", minimum: 0 },
        limit: readLimit,
      },
      ["q"]
    ),
  },
  {
    name: "redmine_get_attachment",
    description: "Get Redmine attachment metadata.",
    inputSchema: objectSchema(
      {
        attachment_id: integer("Attachment ID."),
      },
      ["attachment_id"]
    ),
  },
  {
    name: "redmine_download_attachment",
    description:
      "Download a Redmine attachment and return the content as base64 or UTF-8 text. The response is limited by REDMINE_MCP_ATTACHMENT_MAX_BYTES.",
    inputSchema: objectSchema(
      {
        attachment_id: integer("Attachment ID."),
        filename: {
          type: "string",
          description: "Optional filename override for the Redmine download URL.",
        },
        encoding: {
          type: "string",
          enum: ["base64", "utf8"],
          default: "base64",
        },
        max_bytes: {
          type: "integer",
          minimum: 1,
          description: "Per-call maximum download size in bytes.",
        },
      },
      ["attachment_id"]
    ),
  },
  {
    name: "redmine_upload_attachment",
    description:
      "Upload attachment content to Redmine. Optionally attach the uploaded file to an issue using the returned upload token.",
    inputSchema: objectSchema(
      {
        filename: {
          type: "string",
          minLength: 1,
          description: "File name to send to Redmine. Must not include a path.",
        },
        content_base64: {
          type: "string",
          description: "Base64 encoded file content. Mutually exclusive with content.",
        },
        content: {
          type: "string",
          description: "UTF-8 text content. Mutually exclusive with content_base64.",
        },
        content_type: {
          type: "string",
          description: "Attachment content type recorded on the issue upload entry.",
        },
        description: {
          type: "string",
          description: "Attachment description recorded on the issue upload entry.",
        },
        issue_id: integer("Optional issue ID to attach the uploaded file to."),
        max_bytes: {
          type: "integer",
          minimum: 1,
          description: "Per-call maximum upload size in bytes.",
        },
        ...silentWriteProperties,
      },
      ["filename"]
    ),
  },
  {
    name: "redmine_delete_attachment",
    description:
      "Delete a Redmine attachment. This is destructive and disabled in read-only mode.",
    inputSchema: objectSchema(
      {
        attachment_id: integer("Attachment ID."),
        ...silentWriteProperties,
      },
      ["attachment_id"]
    ),
  },
  {
    name: "redmine_list_issue_relations",
    description: "List relations for a Redmine issue.",
    inputSchema: objectSchema(
      {
        issue_id: integer("Redmine issue ID."),
      },
      ["issue_id"]
    ),
  },
  {
    name: "redmine_get_issue_relation",
    description: "Get one Redmine issue relation.",
    inputSchema: objectSchema(
      {
        relation_id: integer("Issue relation ID."),
      },
      ["relation_id"]
    ),
  },
  {
    name: "redmine_add_issue_relation",
    description: "Add a relation from one Redmine issue to another.",
    inputSchema: objectSchema(
      {
        issue_id: integer("Source Redmine issue ID."),
        issue_to_id: integer("Target Redmine issue ID."),
        relation_type: {
          type: "string",
          enum: [
            "relates",
            "duplicates",
            "duplicated",
            "blocks",
            "blocked",
            "precedes",
            "follows",
            "copied_to",
            "copied_from",
          ],
        },
        delay: integer("Delay in days for precedes/follows relations."),
        ...silentWriteProperties,
      },
      ["issue_id", "issue_to_id", "relation_type"]
    ),
  },
  {
    name: "redmine_delete_issue_relation",
    description: "Delete a Redmine issue relation.",
    inputSchema: objectSchema(
      {
        relation_id: integer("Issue relation ID."),
        ...silentWriteProperties,
      },
      ["relation_id"]
    ),
  },
  {
    name: "redmine_list_checklists",
    description: "List checklist items for an issue using the Redmine Checklists plugin include.",
    inputSchema: objectSchema(
      {
        issue_id: integer("Redmine issue ID."),
      },
      ["issue_id"]
    ),
  },
  {
    name: "redmine_add_checklist_item",
    description: "Add a checklist item to a Redmine issue.",
    inputSchema: objectSchema(
      {
        issue_id: integer("Redmine issue ID."),
        subject: { type: "string", minLength: 1 },
        is_done: { type: "boolean" },
        position: integer("Checklist position."),
        ...silentWriteProperties,
      },
      ["issue_id", "subject"]
    ),
  },
  {
    name: "redmine_update_checklist_item",
    description: "Update a checklist item on a Redmine issue.",
    inputSchema: objectSchema(
      {
        issue_id: integer("Redmine issue ID."),
        checklist_id: integer("Checklist item ID."),
        subject: { type: "string" },
        is_done: { type: "boolean" },
        position: integer("Checklist position."),
        ...silentWriteProperties,
      },
      ["issue_id", "checklist_id"]
    ),
  },
  {
    name: "redmine_delete_checklist_item",
    description: "Delete a checklist item from a Redmine issue.",
    inputSchema: objectSchema(
      {
        issue_id: integer("Redmine issue ID."),
        checklist_id: integer("Checklist item ID."),
        ...silentWriteProperties,
      },
      ["issue_id", "checklist_id"]
    ),
  },
  {
    name: "redmine_list_time_entries",
    description: "List Redmine time entries by issue, project, user, date range, or activity.",
    inputSchema: objectSchema({
      issue_id: integer("Issue ID."),
      project_id: stringOrInteger("Project identifier or numeric ID."),
      user_id: stringOrInteger("User ID or me."),
      activity_id: integer("Time entry activity ID."),
      from: { type: "string", description: "Start date YYYY-MM-DD." },
      to: { type: "string", description: "End date YYYY-MM-DD." },
      offset: { type: "integer", minimum: 0 },
      limit: readLimit,
    }),
  },
  {
    name: "redmine_get_time_entry",
    description: "Get one Redmine time entry.",
    inputSchema: objectSchema(
      {
        time_entry_id: integer("Time entry ID."),
      },
      ["time_entry_id"]
    ),
  },
  {
    name: "redmine_add_time_entry",
    description: "Add a Redmine time entry to an issue or project.",
    inputSchema: objectSchema(
      {
        issue_id: integer("Issue ID."),
        project_id: stringOrInteger("Project identifier or numeric ID."),
        spent_on: { type: "string", description: "Date YYYY-MM-DD." },
        hours: { type: "number", exclusiveMinimum: 0 },
        activity_id: integer("Time entry activity ID."),
        comments: { type: "string" },
        user_id: integer("User ID."),
        ...silentWriteProperties,
      },
      ["hours"]
    ),
  },
  {
    name: "redmine_update_time_entry",
    description: "Update a Redmine time entry.",
    inputSchema: objectSchema(
      {
        time_entry_id: integer("Time entry ID."),
        fields: {
          type: "object",
          additionalProperties: true,
          description: "Time entry fields such as hours, spent_on, activity_id, comments, issue_id, or project_id.",
        },
        ...silentWriteProperties,
      },
      ["time_entry_id", "fields"]
    ),
  },
  {
    name: "redmine_delete_time_entry",
    description: "Delete a Redmine time entry.",
    inputSchema: objectSchema(
      {
        time_entry_id: integer("Time entry ID."),
        ...silentWriteProperties,
      },
      ["time_entry_id"]
    ),
  },
  {
    name: "redmine_list_versions",
    description: "List versions for a Redmine project.",
    inputSchema: objectSchema(
      {
        project_id: stringOrInteger("Project identifier or numeric ID."),
      },
      ["project_id"]
    ),
  },
  {
    name: "redmine_get_version",
    description: "Get one Redmine version.",
    inputSchema: objectSchema(
      {
        version_id: integer("Version ID."),
      },
      ["version_id"]
    ),
  },
  {
    name: "redmine_create_version",
    description: "Create a Redmine project version.",
    inputSchema: objectSchema(
      {
        project_id: stringOrInteger("Project identifier or numeric ID."),
        name: { type: "string", minLength: 1 },
        description: { type: "string" },
        effective_date: { type: "string", description: "Date YYYY-MM-DD." },
        status: { type: "string", enum: ["open", "locked", "closed"] },
        sharing: {
          type: "string",
          enum: ["none", "descendants", "hierarchy", "tree", "system"],
        },
        wiki_page_title: { type: "string" },
        ...silentWriteProperties,
      },
      ["project_id", "name"]
    ),
  },
  {
    name: "redmine_update_version",
    description: "Update a Redmine version.",
    inputSchema: objectSchema(
      {
        version_id: integer("Version ID."),
        fields: {
          type: "object",
          additionalProperties: true,
          description: "Version fields such as name, description, status, sharing, effective_date, or wiki_page_title.",
        },
        ...silentWriteProperties,
      },
      ["version_id", "fields"]
    ),
  },
  {
    name: "redmine_delete_version",
    description: "Delete a Redmine version.",
    inputSchema: objectSchema(
      {
        version_id: integer("Version ID."),
        ...silentWriteProperties,
      },
      ["version_id"]
    ),
  },
  {
    name: "redmine_list_watchers",
    description: "List watchers for a Redmine issue.",
    inputSchema: objectSchema(
      {
        issue_id: integer("Redmine issue ID."),
      },
      ["issue_id"]
    ),
  },
  {
    name: "redmine_add_watcher",
    description: "Add a watcher to a Redmine issue.",
    inputSchema: objectSchema(
      {
        issue_id: integer("Redmine issue ID."),
        user_id: integer("User ID to add as watcher."),
        ...silentWriteProperties,
      },
      ["issue_id", "user_id"]
    ),
  },
  {
    name: "redmine_remove_watcher",
    description: "Remove a watcher from a Redmine issue.",
    inputSchema: objectSchema(
      {
        issue_id: integer("Redmine issue ID."),
        user_id: integer("Watcher user ID."),
        ...silentWriteProperties,
      },
      ["issue_id", "user_id"]
    ),
  },
  {
    name: "redmine_list_time_entry_activities",
    description: "List Redmine time entry activities.",
    inputSchema: objectSchema({}),
  },
  {
    name: "redmine_list_projects",
    description: "List Redmine projects.",
    inputSchema: objectSchema({
      include: {
        type: "array",
        description: "Associated project data to include.",
        items: {
          type: "string",
          enum: PROJECT_INCLUDES,
        },
      },
      offset: { type: "integer", minimum: 0 },
      limit: readLimit,
    }),
  },
  {
    name: "redmine_get_project",
    description: "Get one Redmine project by numeric ID or identifier.",
    inputSchema: objectSchema(
      {
        project_id: stringOrInteger("Project identifier or numeric ID."),
        include: {
          type: "array",
          description: "Associated project data to include.",
          items: {
            type: "string",
            enum: PROJECT_INCLUDES,
          },
        },
      },
      ["project_id"]
    ),
  },
  {
    name: "redmine_list_project_memberships",
    description: "List memberships for one Redmine project. Read-only.",
    inputSchema: objectSchema(
      {
        project_id: stringOrInteger("Project identifier or numeric ID."),
        offset: { type: "integer", minimum: 0 },
        limit: readLimit,
      },
      ["project_id"]
    ),
  },
  {
    name: "redmine_get_project_membership",
    description: "Get one Redmine project membership. Read-only.",
    inputSchema: objectSchema(
      {
        membership_id: integer("Project membership ID."),
      },
      ["membership_id"]
    ),
  },
  {
    name: "redmine_list_wiki_pages",
    description: "List wiki pages for one Redmine project. Read-only.",
    inputSchema: objectSchema(
      {
        project_id: stringOrInteger("Project identifier or numeric ID."),
      },
      ["project_id"]
    ),
  },
  {
    name: "redmine_get_wiki_page",
    description: "Get one Redmine wiki page, optionally at a specific version. Read-only.",
    inputSchema: objectSchema(
      {
        project_id: stringOrInteger("Project identifier or numeric ID."),
        title: {
          type: "string",
          minLength: 1,
          description: "Wiki page title.",
        },
        version: integer("Optional wiki page version."),
        include: {
          type: "array",
          description: "Associated wiki page data to include.",
          items: {
            type: "string",
            enum: WIKI_INCLUDES,
          },
        },
      },
      ["project_id", "title"]
    ),
  },
  {
    name: "redmine_list_issue_statuses",
    description: "List Redmine issue statuses.",
    inputSchema: objectSchema({}),
  },
  {
    name: "redmine_list_trackers",
    description: "List Redmine trackers.",
    inputSchema: objectSchema({}),
  },
  {
    name: "redmine_list_issue_priorities",
    description: "List Redmine issue priorities.",
    inputSchema: objectSchema({}),
  },
  {
    name: "redmine_list_issue_categories",
    description: "List issue categories for one Redmine project.",
    inputSchema: objectSchema(
      {
        project_id: stringOrInteger("Project identifier or numeric ID."),
      },
      ["project_id"]
    ),
  },
  {
    name: "redmine_list_custom_fields",
    description: "List Redmine custom field definitions. Redmine requires admin privileges.",
    inputSchema: objectSchema({}),
  },
  {
    name: "redmine_list_queries",
    description: "List saved Redmine issue queries visible to the API user.",
    inputSchema: objectSchema({
      project_id: stringOrInteger("Optional project identifier or numeric ID."),
      offset: { type: "integer", minimum: 0 },
      limit: readLimit,
    }),
  },
  {
    name: "redmine_list_users",
    description: "List Redmine users, optionally filtered by name.",
    inputSchema: objectSchema({
      name: { type: "string" },
      group_id: integer("Group ID."),
      status: integer("User status."),
      offset: { type: "integer", minimum: 0 },
      limit: readLimit,
    }),
  },
  {
    name: "redmine_get_current_user",
    description: "Get the Redmine user associated with the configured API key.",
    inputSchema: objectSchema({
      include: {
        type: "array",
        description: "Optional user associations to include.",
        items: {
          type: "string",
          enum: ["memberships", "groups"],
        },
      },
    }),
  },
];

const handlers = {
  async redmine_get_issue(client, args) {
    const issueId = required(args, "issue_id");
    return unwrap(
      await client.request("GET", `/issues/${encodeURIComponent(issueId)}.json`, {
        query: {
          include: issueIncludes(client, args.include),
        },
      })
    );
  },

  async redmine_list_issues(client, args) {
    return unwrap(await client.request("GET", "/issues.json", { query: filterArgs(args) }));
  },

  async redmine_create_issue(client, args) {
    const issue = {
      ...optionalObjectArg(args, "fields"),
      ...pickDefined(args, [
        "project_id",
        "subject",
        "description",
        "tracker_id",
        "status_id",
        "priority_id",
        "assigned_to_id",
        "category_id",
        "fixed_version_id",
        "parent_issue_id",
        "start_date",
        "due_date",
        "done_ratio",
        "estimated_hours",
        "custom_fields",
        "watcher_user_ids",
      ]),
    };
    const response = await client.request("POST", "/issues.json", {
      query: writeQuery(client, args),
      body: { issue },
    });
    return writeResult(client, args, "create_issue", targetFrom(issue), response);
  },

  async redmine_update_issue(client, args) {
    const issueId = required(args, "issue_id");
    const fields = objectArg(args, "fields");
    const response = await client.request("PUT", `/issues/${encodeURIComponent(issueId)}.json`, {
      query: writeQuery(client, args),
      body: { issue: fields },
    });
    return writeResult(client, args, "update_issue", { issue_id: issueId }, response);
  },

  async redmine_delete_issue(client, args) {
    const issueId = required(args, "issue_id");
    const response = await client.request("DELETE", `/issues/${encodeURIComponent(issueId)}.json`, {
      query: writeQuery(client, args),
    });
    return writeResult(client, args, "delete_issue", { issue_id: issueId }, response);
  },

  async redmine_search(client, args) {
    return unwrap(await client.request("GET", "/search.json", { query: filterArgs(args) }));
  },

  async redmine_get_attachment(client, args) {
    const attachmentId = required(args, "attachment_id");
    return unwrap(
      await client.request("GET", `/attachments/${encodeURIComponent(attachmentId)}.json`)
    );
  },

  async redmine_download_attachment(client, args) {
    const attachmentId = required(args, "attachment_id");
    const metadata = await handlers.redmine_get_attachment(client, args);
    const attachment = metadata && metadata.attachment ? metadata.attachment : {};
    const filename = args.filename || attachment.filename;
    if (!filename) {
      throw new InputError("filename is required when attachment metadata does not include one");
    }

    const maxBytes = attachmentMaxBytes(client, args);
    const declaredSize = Number(attachment.filesize);
    if (Number.isFinite(declaredSize) && declaredSize > maxBytes) {
      throw new InputError(
        `Attachment ${attachmentId} is ${declaredSize} bytes, exceeding max_bytes ${maxBytes}`
      );
    }

    const response = await client.request(
      "GET",
      `/attachments/download/${encodeURIComponent(attachmentId)}/${encodeURIComponent(filename)}`,
      {
        accept: "*/*",
        responseType: "buffer",
      }
    );
    const bytes = response.body;
    if (bytes.length > maxBytes) {
      throw new InputError(
        `Downloaded attachment ${attachmentId} is ${bytes.length} bytes, exceeding max_bytes ${maxBytes}`
      );
    }

    const encoding = args.encoding || "base64";
    const result = {
      attachment_id: attachmentId,
      filename,
      content_type:
        response.headers.get("content-type") || attachment.content_type || "application/octet-stream",
      size: bytes.length,
      encoding,
    };
    if (encoding === "utf8") {
      result.content = bytes.toString("utf8");
    } else {
      result.content_base64 = bytes.toString("base64");
    }
    return result;
  },

  async redmine_upload_attachment(client, args) {
    const filename = attachmentFilename(args);
    const bytes = attachmentUploadBytes(args);
    const maxBytes = attachmentMaxBytes(client, args);
    if (bytes.length > maxBytes) {
      throw new InputError(
        `Attachment upload is ${bytes.length} bytes, exceeding max_bytes ${maxBytes}`
      );
    }

    const uploadResponse = await client.request("POST", "/uploads.json", {
      query: { filename },
      rawBody: bytes,
      contentType: "application/octet-stream",
    });
    const token = uploadResponse.body && uploadResponse.body.upload && uploadResponse.body.upload.token;
    if (!token) {
      throw new Error("Redmine upload response did not include upload.token");
    }

    const upload = {
      token,
      filename,
      ...pickDefined(args, ["description", "content_type"]),
    };
    const issueId = args.issue_id;
    if (issueId !== undefined && issueId !== null && issueId !== "") {
      const attachResponse = await client.request(
        "PUT",
        `/issues/${encodeURIComponent(issueId)}.json`,
        {
          query: writeQuery(client, args),
          body: {
            issue: {
              uploads: [upload],
            },
          },
        }
      );
      return attachmentWriteResult(
        client,
        args,
        "upload_attachment",
        { issue_id: issueId, filename, token },
        attachResponse,
        upload
      );
    }

    return attachmentWriteResult(
      client,
      args,
      "upload_attachment",
      { filename, token },
      uploadResponse,
      upload
    );
  },

  async redmine_delete_attachment(client, args) {
    const attachmentId = required(args, "attachment_id");
    const response = await client.request(
      "DELETE",
      `/attachments/${encodeURIComponent(attachmentId)}.json`,
      { query: writeQuery(client, args) }
    );
    return writeResult(client, args, "delete_attachment", { attachment_id: attachmentId }, response);
  },

  async redmine_list_issue_relations(client, args) {
    const issueId = required(args, "issue_id");
    return unwrap(
      await client.request("GET", `/issues/${encodeURIComponent(issueId)}/relations.json`)
    );
  },

  async redmine_get_issue_relation(client, args) {
    const relationId = required(args, "relation_id");
    return unwrap(await client.request("GET", `/relations/${encodeURIComponent(relationId)}.json`));
  },

  async redmine_add_issue_relation(client, args) {
    const issueId = required(args, "issue_id");
    const issueToId = required(args, "issue_to_id");
    const relationType = required(args, "relation_type");
    const relation = {
      issue_to_id: issueToId,
      relation_type: relationType,
      ...pickDefined(args, ["delay"]),
    };
    const response = await client.request(
      "POST",
      `/issues/${encodeURIComponent(issueId)}/relations.json`,
      {
        query: writeQuery(client, args),
        body: { relation },
      }
    );
    return writeResult(
      client,
      args,
      "add_issue_relation",
      { issue_id: issueId, issue_to_id: issueToId, relation_type: relationType },
      response
    );
  },

  async redmine_delete_issue_relation(client, args) {
    const relationId = required(args, "relation_id");
    const response = await client.request(
      "DELETE",
      `/relations/${encodeURIComponent(relationId)}.json`,
      { query: writeQuery(client, args) }
    );
    return writeResult(client, args, "delete_issue_relation", { relation_id: relationId }, response);
  },

  async redmine_list_checklists(client, args) {
    const issueId = required(args, "issue_id");
    const response = await client.request("GET", `/issues/${encodeURIComponent(issueId)}.json`, {
      query: { include: "checklists" },
    });
    const issue = response.body && response.body.issue ? response.body.issue : {};
    return {
      issue_id: issueId,
      checklists: issue.checklists || [],
    };
  },

  async redmine_add_checklist_item(client, args) {
    const issueId = required(args, "issue_id");
    const checklist = pickDefined(args, ["subject", "is_done", "position"]);
    const response = await updateChecklist(client, args, issueId, checklist);
    return writeResult(client, args, "add_checklist_item", { issue_id: issueId }, response);
  },

  async redmine_update_checklist_item(client, args) {
    const issueId = required(args, "issue_id");
    const checklistId = required(args, "checklist_id");
    const checklist = {
      id: checklistId,
      ...pickDefined(args, ["subject", "is_done", "position"]),
    };
    const response = await updateChecklist(client, args, issueId, checklist);
    return writeResult(
      client,
      args,
      "update_checklist_item",
      { issue_id: issueId, checklist_id: checklistId },
      response
    );
  },

  async redmine_delete_checklist_item(client, args) {
    const issueId = required(args, "issue_id");
    const checklistId = required(args, "checklist_id");
    const response = await updateChecklist(client, args, issueId, {
      id: checklistId,
      _destroy: true,
    });
    return writeResult(
      client,
      args,
      "delete_checklist_item",
      { issue_id: issueId, checklist_id: checklistId },
      response
    );
  },

  async redmine_list_time_entries(client, args) {
    return unwrap(await client.request("GET", "/time_entries.json", { query: filterArgs(args) }));
  },

  async redmine_get_time_entry(client, args) {
    const timeEntryId = required(args, "time_entry_id");
    return unwrap(
      await client.request("GET", `/time_entries/${encodeURIComponent(timeEntryId)}.json`)
    );
  },

  async redmine_add_time_entry(client, args) {
    if (args.issue_id === undefined && args.project_id === undefined) {
      throw new InputError("Either issue_id or project_id is required");
    }
    const timeEntry = pickDefined(args, [
      "issue_id",
      "project_id",
      "spent_on",
      "hours",
      "activity_id",
      "comments",
      "user_id",
    ]);
    const response = await client.request("POST", "/time_entries.json", {
      query: writeQuery(client, args),
      body: { time_entry: timeEntry },
    });
    return writeResult(client, args, "add_time_entry", targetFrom(timeEntry), response);
  },

  async redmine_update_time_entry(client, args) {
    const timeEntryId = required(args, "time_entry_id");
    const fields = objectArg(args, "fields");
    const response = await client.request(
      "PUT",
      `/time_entries/${encodeURIComponent(timeEntryId)}.json`,
      {
        query: writeQuery(client, args),
        body: { time_entry: fields },
      }
    );
    return writeResult(client, args, "update_time_entry", { time_entry_id: timeEntryId }, response);
  },

  async redmine_delete_time_entry(client, args) {
    const timeEntryId = required(args, "time_entry_id");
    const response = await client.request(
      "DELETE",
      `/time_entries/${encodeURIComponent(timeEntryId)}.json`,
      { query: writeQuery(client, args) }
    );
    return writeResult(client, args, "delete_time_entry", { time_entry_id: timeEntryId }, response);
  },

  async redmine_list_versions(client, args) {
    const projectId = required(args, "project_id");
    return unwrap(
      await client.request("GET", `/projects/${encodeURIComponent(projectId)}/versions.json`)
    );
  },

  async redmine_get_version(client, args) {
    const versionId = required(args, "version_id");
    return unwrap(await client.request("GET", `/versions/${encodeURIComponent(versionId)}.json`));
  },

  async redmine_create_version(client, args) {
    const projectId = required(args, "project_id");
    const version = pickDefined(args, [
      "name",
      "description",
      "effective_date",
      "status",
      "sharing",
      "wiki_page_title",
    ]);
    const response = await client.request(
      "POST",
      `/projects/${encodeURIComponent(projectId)}/versions.json`,
      {
        query: writeQuery(client, args),
        body: { version },
      }
    );
    return writeResult(client, args, "create_version", { project_id: projectId }, response);
  },

  async redmine_update_version(client, args) {
    const versionId = required(args, "version_id");
    const fields = objectArg(args, "fields");
    const response = await client.request("PUT", `/versions/${encodeURIComponent(versionId)}.json`, {
      query: writeQuery(client, args),
      body: { version: fields },
    });
    return writeResult(client, args, "update_version", { version_id: versionId }, response);
  },

  async redmine_delete_version(client, args) {
    const versionId = required(args, "version_id");
    const response = await client.request(
      "DELETE",
      `/versions/${encodeURIComponent(versionId)}.json`,
      { query: writeQuery(client, args) }
    );
    return writeResult(client, args, "delete_version", { version_id: versionId }, response);
  },

  async redmine_list_watchers(client, args) {
    const issueId = required(args, "issue_id");
    const response = await client.request("GET", `/issues/${encodeURIComponent(issueId)}.json`, {
      query: { include: "watchers" },
    });
    const issue = response.body && response.body.issue ? response.body.issue : {};
    return {
      issue_id: issueId,
      watchers: issue.watchers || [],
    };
  },

  async redmine_add_watcher(client, args) {
    const issueId = required(args, "issue_id");
    const userId = required(args, "user_id");
    const response = await client.request(
      "POST",
      `/issues/${encodeURIComponent(issueId)}/watchers.json`,
      {
        query: writeQuery(client, args),
        body: { user_id: userId },
      }
    );
    return writeResult(client, args, "add_watcher", { issue_id: issueId, user_id: userId }, response);
  },

  async redmine_remove_watcher(client, args) {
    const issueId = required(args, "issue_id");
    const userId = required(args, "user_id");
    const response = await client.request(
      "DELETE",
      `/issues/${encodeURIComponent(issueId)}/watchers/${encodeURIComponent(userId)}.json`,
      { query: writeQuery(client, args) }
    );
    return writeResult(
      client,
      args,
      "remove_watcher",
      { issue_id: issueId, user_id: userId },
      response
    );
  },

  async redmine_list_time_entry_activities(client) {
    return unwrap(await client.request("GET", "/enumerations/time_entry_activities.json"));
  },

  async redmine_list_projects(client, args) {
    return unwrap(await client.request("GET", "/projects.json", { query: filterArgs(args) }));
  },

  async redmine_get_project(client, args) {
    const projectId = required(args, "project_id");
    return unwrap(
      await client.request("GET", `/projects/${encodeURIComponent(projectId)}.json`, {
        query: {
          include: args.include,
        },
      })
    );
  },

  async redmine_list_project_memberships(client, args) {
    const projectId = required(args, "project_id");
    return unwrap(
      await client.request("GET", `/projects/${encodeURIComponent(projectId)}/memberships.json`, {
        query: pickDefined(args, ["offset", "limit"]),
      })
    );
  },

  async redmine_get_project_membership(client, args) {
    const membershipId = required(args, "membership_id");
    return unwrap(
      await client.request("GET", `/memberships/${encodeURIComponent(membershipId)}.json`)
    );
  },

  async redmine_list_wiki_pages(client, args) {
    const projectId = required(args, "project_id");
    return unwrap(
      await client.request("GET", `/projects/${encodeURIComponent(projectId)}/wiki/index.json`)
    );
  },

  async redmine_get_wiki_page(client, args) {
    const projectId = required(args, "project_id");
    const title = required(args, "title");
    const version = args.version;
    const versionPath =
      version !== undefined && version !== null && version !== ""
        ? `/${encodeURIComponent(version)}`
        : "";
    return unwrap(
      await client.request(
        "GET",
        `/projects/${encodeURIComponent(projectId)}/wiki/${encodeURIComponent(title)}${versionPath}.json`,
        {
          query: {
            include: wikiIncludes(client, args.include),
          },
        }
      )
    );
  },

  async redmine_list_issue_statuses(client) {
    return unwrap(await client.request("GET", "/issue_statuses.json"));
  },

  async redmine_list_trackers(client) {
    return unwrap(await client.request("GET", "/trackers.json"));
  },

  async redmine_list_issue_priorities(client) {
    return unwrap(await client.request("GET", "/enumerations/issue_priorities.json"));
  },

  async redmine_list_issue_categories(client, args) {
    const projectId = required(args, "project_id");
    return unwrap(
      await client.request(
        "GET",
        `/projects/${encodeURIComponent(projectId)}/issue_categories.json`
      )
    );
  },

  async redmine_list_custom_fields(client) {
    return unwrap(await client.request("GET", "/custom_fields.json"));
  },

  async redmine_list_queries(client, args) {
    return unwrap(await client.request("GET", "/queries.json", { query: filterArgs(args) }));
  },

  async redmine_list_users(client, args) {
    return unwrap(await client.request("GET", "/users.json", { query: filterArgs(args) }));
  },

  async redmine_get_current_user(client, args) {
    return unwrap(
      await client.request("GET", "/users/current.json", { query: filterArgs(args) })
    );
  },
};

export function listTools(config = {}) {
  return TOOLS.filter((tool) => {
    if (disabledFeatureForTool(config, tool.name)) {
      return false;
    }
    if (DELETE_TOOLS.has(tool.name) && !config.enableDeletes) {
      return false;
    }
    if (config.readOnly && WRITE_TOOLS.has(tool.name)) {
      return false;
    }
    return true;
  });
}

export async function callTool(client, name, args = {}) {
  const handler = handlers[name];
  if (!handler) {
    throw new InputError(`Unknown tool: ${name}`);
  }
  const disabledFeature = disabledFeatureForTool(client.config, name);
  if (disabledFeature) {
    throw new Error(`${FEATURE_ENV[disabledFeature]} is enabled; tool ${name} is disabled`);
  }
  if (DELETE_TOOLS.has(name) && !(client.config && client.config.enableDeletes)) {
    throw new Error(
      `REDMINE_MCP_ENABLE_DELETES is not enabled; delete/remove tool ${name} is disabled`
    );
  }
  if (client.config && client.config.readOnly && WRITE_TOOLS.has(name)) {
    throw new Error(`REDMINE_MCP_READ_ONLY is enabled; write tool ${name} is disabled`);
  }
  return handler(client, args || {});
}

export function toolErrorPayload(error) {
  if (error instanceof RedmineError) {
    return {
      ok: false,
      error: error.message,
      status: error.status,
      method: error.method,
      path: error.path,
      details: error.details,
    };
  }
  return {
    ok: false,
    error: error.message || String(error),
  };
}

function attachmentWriteResult(client, args, operation, target, response, upload) {
  if (isSilentWrite(client, args)) {
    return {
      ok: true,
      operation,
      target,
      status: response.status,
      upload,
    };
  }
  return {
    ok: true,
    operation,
    target,
    status: response.status,
    upload,
    response: response.body,
  };
}

function attachmentMaxBytes(client, args) {
  const configured = Number(client.config && client.config.attachmentMaxBytes);
  const requested = Number(args.max_bytes);
  const maxBytes = Number.isFinite(requested) && requested > 0 ? requested : configured;
  return Number.isFinite(maxBytes) && maxBytes > 0 ? maxBytes : DEFAULT_ATTACHMENT_MAX_BYTES;
}

function attachmentFilename(args) {
  const filename = String(required(args, "filename"));
  if (filename.includes("/") || filename.includes("\\") || filename === "." || filename === "..") {
    throw new InputError("filename must be a file name, not a path");
  }
  return filename;
}

function attachmentUploadBytes(args) {
  const hasBase64 = args.content_base64 !== undefined && args.content_base64 !== null;
  const hasText = args.content !== undefined && args.content !== null;
  if (hasBase64 === hasText) {
    throw new InputError("Exactly one of content_base64 or content is required");
  }
  if (hasText) {
    return Buffer.from(String(args.content), "utf8");
  }

  const normalized = String(args.content_base64).replace(/\s/g, "");
  if (!/^[A-Za-z0-9+/]*={0,2}$/.test(normalized) || normalized.length % 4 !== 0) {
    throw new InputError("content_base64 must be valid base64");
  }
  return Buffer.from(normalized, "base64");
}

async function updateChecklist(client, args, issueId, checklist) {
  return client.request("PUT", `/issues/${encodeURIComponent(issueId)}.json`, {
    query: writeQuery(client, args),
    body: {
      issue: {
        checklists_attributes: [checklist],
      },
    },
  });
}

function writeResult(client, args, operation, target, response) {
  if (isSilentWrite(client, args)) {
    return {
      ok: true,
      operation,
      target,
      status: response.status,
    };
  }
  return {
    ok: true,
    operation,
    target,
    status: response.status,
    response: response.body,
  };
}

function isSilentWrite(client, args) {
  if (args.silent !== undefined) {
    return parseBoolean(args.silent, false);
  }
  return Boolean(client.config && client.config.silentWrites);
}

function writeQuery(client, args) {
  if (args.notify !== undefined) {
    return { notify: parseBoolean(args.notify, true) ? "true" : "false" };
  }
  if (isSilentWrite(client, args)) {
    return { notify: "false" };
  }
  return {};
}

function filterArgs(args) {
  const ignored = new Set(["silent", "notify", "fields"]);
  const result = {};
  for (const [key, value] of Object.entries(args || {})) {
    if (!ignored.has(key) && value !== undefined && value !== null && value !== "") {
      result[key] = value;
    }
  }
  return result;
}

function issueIncludes(client, requestedIncludes) {
  const disabled = disabledFeatures(client.config);
  const includes =
    Array.isArray(requestedIncludes) && requestedIncludes.length
      ? requestedIncludes
      : DEFAULT_ISSUE_INCLUDES;
  const filtered = includes.filter((include) => {
    if (include === "checklists") {
      return !disabled.checklists;
    }
    if (include === "relations") {
      return !disabled.relations;
    }
    if (include === "watchers") {
      return !disabled.watchers;
    }
    if (include === "attachments") {
      return !disabled.attachments;
    }
    return true;
  });
  return filtered.length ? filtered : undefined;
}

function wikiIncludes(client, requestedIncludes) {
  const disabled = disabledFeatures(client.config);
  const includes = Array.isArray(requestedIncludes) ? requestedIncludes : [];
  const filtered = includes.filter((include) => {
    if (include === "attachments") {
      return !disabled.attachments;
    }
    return true;
  });
  return filtered.length ? filtered : undefined;
}

function disabledFeatureForTool(config = {}, toolName) {
  const disabled = disabledFeatures(config);
  for (const [feature, tools] of Object.entries(FEATURE_TOOLS)) {
    if (disabled[feature] && tools.has(toolName)) {
      return feature;
    }
  }
  return null;
}

function disabledFeatures(config = {}) {
  return {
    attachments: false,
    checklists: false,
    relations: false,
    timeEntries: false,
    versions: false,
    wiki: false,
    watchers: false,
    ...(config.disabledFeatures || {}),
  };
}

function pickDefined(source, keys) {
  const result = {};
  for (const key of keys) {
    if (source[key] !== undefined && source[key] !== null) {
      result[key] = source[key];
    }
  }
  return result;
}

function required(args, key) {
  if (args[key] === undefined || args[key] === null || args[key] === "") {
    throw new InputError(`${key} is required`);
  }
  return args[key];
}

function objectArg(args, key) {
  const value = required(args, key);
  if (typeof value !== "object" || Array.isArray(value)) {
    throw new InputError(`${key} must be an object`);
  }
  return value;
}

function optionalObjectArg(args, key) {
  const value = args[key];
  if (value === undefined || value === null) {
    return {};
  }
  if (typeof value !== "object" || Array.isArray(value)) {
    throw new InputError(`${key} must be an object`);
  }
  return value;
}

function unwrap(response) {
  return response.body;
}

function targetFrom(value) {
  return pickDefined(value, ["issue_id", "project_id", "user_id"]);
}

function objectSchema(properties, required = []) {
  return {
    type: "object",
    additionalProperties: false,
    properties,
    required,
  };
}

function integer(description) {
  return {
    type: "integer",
    description,
  };
}

function stringOrInteger(description) {
  return {
    oneOf: [{ type: "string" }, { type: "integer" }],
    description,
  };
}
