# Security Policy

## Supported Versions

Only the latest stable release receives security fixes.

| Version | Supported |
|---------|-----------|
| 1.2.x   | Yes       |
| < 1.2   | No        |

## Reporting a Vulnerability

**Please do not open a public GitHub issue for security vulnerabilities.**

Instead, report them privately via one of these channels:

- **GitHub Private Advisory**: [Report a vulnerability](https://github.com/drunkleen/leenfetch/security/advisories/new)
- **Email**: snape@drunkleen.com

Include as much detail as possible:
- A description of the vulnerability and its potential impact
- Steps to reproduce
- Affected version(s)
- Any suggested fix, if you have one

You can expect an acknowledgement within **48 hours** and a resolution or status update within **7 days**.

## Scope

LeenFetch is a read-only system information tool. It does not accept network connections, write to disk (except config files), or run with elevated privileges. That said, the following are in scope:

- Command injection via crafted config values or environment variables
- Path traversal in config or ASCII file loading
- Information disclosure beyond what the user explicitly enables
- Unsafe handling of data from remote SSH hosts

## Out of Scope

- Issues in third-party dependencies (report those upstream)
- Denial of service via resource exhaustion on the local machine
- Social engineering
