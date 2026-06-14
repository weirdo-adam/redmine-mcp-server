# Security Policy

## Reporting a Vulnerability

Please do not disclose vulnerabilities publicly before they have been reviewed.
If GitHub private vulnerability reporting is enabled for this repository, use
that channel. Otherwise, open a minimal GitHub issue asking for a maintainer
contact path, without sharing secrets, exploit details, private Redmine data, or
internal URLs.

Useful reports include the affected server version, Redmine version, relevant
configuration flags, and a minimal sanitized reproduction.

## Credentials and Redmine Data

Do not include `REDMINE_API_KEY` values, private issue data, or internal Redmine
URLs in public issues, pull requests, screenshots, or logs.

Use a Redmine API key with the smallest practical permission scope. Set
`REDMINE_MCP_READ_ONLY=true` when the agent should inspect Redmine without
making changes.
