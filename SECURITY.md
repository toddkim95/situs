# Security Policy

## Reporting A Vulnerability

Please do not open a public issue for a security-sensitive report.

Use GitHub's private vulnerability reporting or contact the maintainers
privately. Include:

- affected version or commit
- operating system and shell
- reproduction steps
- expected impact
- any suggested mitigation

The project is early, so response times may vary, but security reports will be
handled before ordinary feature work when they affect user safety.

## Scope

Interesting security areas for Situs include:

- shell command construction
- zsh widget output parsing
- history storage and deletion
- Atuin SQLite import behavior
- terminal escape handling

Situs imports Atuin history read-only and should not mutate Atuin's database.
