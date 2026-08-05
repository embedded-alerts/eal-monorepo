# Zed dependency graph

This repository family uses the short `eal-*` names as the only canonical package identities.

## Required edges

| Consumer role | Required Zed dependencies |
| --- | --- |
| `eal-clients` | `eal-interfaces` |
| `eal-libs` | `eal-interfaces` |
| API and web/UI servers | `eal-interfaces`, `eal-libs`, `eal-sync`; backend-capable services also use `shared-auth-clients` |
| `eal-cli` | `eal-clients`, `eal-interfaces`, `eal-libs` |
| planned `eal-mcp-server.rs` | `eal-clients`, `eal-interfaces`, `eal-libs`, `eal-sync`, `shared-auth-clients` |
| planned `eal-e2e` | `eal-clients`, `eal-interfaces`, `eal-libs`, `eal-cli` |
| `eal-monorepo` | the complete shared graph: clients, interfaces, libs, CLI, sync, and shared auth |

The root `.zpkg.toml` is the executable contract for the monorepo row. Repositories that implement a UI or backend must depend on `eal-sync`; backend services that authenticate users should depend on `shared-auth/shared-auth-clients`.

## Git submodule interoperability

Zed packages are the dependency mechanism. Git submodules remain supported for intentionally embedded source, fixtures, or transition work:

```bash
git submodule update --init --recursive
zed install --git-submodules
```

Do not represent the same repository as both a Zed dependency and a gitlink. Retained gitlinks must be classified in `.zed-submodules.tsv`; use `zed overtake --git-submodules` when deliberately migrating submodules into Zed dependencies. Never commit `.vendor/.zed` or `zed_modules`.

## Naming and consolidation

Do not add full-name package aliases such as `embedded-alerts-clients`. The long-name repositories are compatibility history only; all new issues, releases, dependencies, and submodules target the short-name repositories.
