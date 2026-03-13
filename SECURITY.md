# Security Policy

## Reporting a Vulnerability

If you discover a security vulnerability in MAESMA, please report it responsibly:

1. **Do not** open a public GitHub issue
2. Email the maintainers with details of the vulnerability
3. Include steps to reproduce, potential impact, and any suggested fixes

We will acknowledge receipt within 48 hours and provide a timeline for resolution.

## Scope

MAESMA is a research simulation platform. Security considerations include:

- **API server** (`maesma-api`): The Axum REST/WebSocket server should not be exposed to untrusted networks without authentication
- **Data ingestion**: External data sources (STAC catalogs, observation feeds) should be validated before processing
- **Federation** (`maesma-federation`): A2A peer connections should use mutual TLS in production deployments
- **CLI**: Command-line inputs are validated via clap; file paths are sanitized

## Supported Versions

| Version | Supported |
| ------- | --------- |
| 0.1.x   | Yes       |
