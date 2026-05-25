# Open-Source Documentation Pass

Date: 2026-05-24

Goal: prepare Situs for a public GitHub repository by matching the baseline
documentation shape users expect from modern CLI tools.

## References Checked

- Atuin: strong problem statement, feature list, quickstart, docs links, shell
  support, sync/import story.
- zoxide: concise getting-started examples, installation matrix, shell setup,
  configuration and integrations links.
- fzf: shell integration notes, key binding documentation, environment-driven
  customization.
- GitHub CLI: install sections per platform, source build notes, contributing,
  code of conduct, security policy, release verification emphasis.
- fd and bat: practical install notes, configuration details, troubleshooting
  and platform-specific caveats.
- GitHub community profile docs: README, LICENSE, CONTRIBUTING, CODE_OF_CONDUCT,
  SECURITY, issue templates, and PR template improve public repository health.

## Decisions

- Keep README as a fast public entry point: problem, quickstart, controls,
  core commands, and links.
- Move detail into `docs/`:
  - installation
  - usage
  - configuration
  - comparison
  - troubleshooting
  - development
  - release checklist
- Add GitHub community files and CI before public launch.
- Keep the product language centered on `command cwd resolver`.
- Add `repository` metadata to `Cargo.toml` so `cargo package` is clean.

## Follow-Ups

- Replace install URLs if the public GitHub owner/name changes.
- Add screenshots or an asciinema demo before broader announcement.
- Add Homebrew and binary release docs once packaging exists.
