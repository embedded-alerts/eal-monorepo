# Zed fleet and project synchronization

Status date: 2026-08-05

This document is the durable cross-system status record for the canonical `embedded-alerts` source fleet. It keeps GitHub repositories, GitHub Project #1, and the Linear project `github.com/embedded-alerts` aligned without creating a second package namespace.

## Canonical package graph

| Consumer | Required Zed dependencies |
| --- | --- |
| `eal-clients` | `embedded-alerts/eal-interfaces` |
| `eal-libs` | `embedded-alerts/eal-interfaces` |
| `eal-sync` | `embedded-alerts/eal-interfaces` |
| `eal-cli` | `eal-clients`, `eal-interfaces`, `eal-libs` |
| API and web servers | `eal-interfaces`, `eal-libs`, `eal-sync`, `shared-auth/shared-auth-clients` |
| `eal-monorepo` | clients, interfaces, libs, CLI, sync, and shared-auth clients |
| planned `eal-mcp-server.rs` | clients, interfaces, libs, CLI, sync, and shared-auth clients |
| planned `eal-e2e` | clients, interfaces, libs, and CLI |

Dependencies materialize under `.vendor/.zed`. Generated dependency trees are not committed or published. `.zpkg.lock` is generated only by a real successful resolver run; it is never fabricated from repository metadata.

## Completed delivery

- `eal-monorepo#4` completed the canonical short-name dependency graph.
- `eal-sync#3` added the missing Zed package identity and restored the dependencies required by the existing service source.
- API, Mash, Leptos, and Dioxus packages consume interfaces, libs, sync, and shared-auth clients.
- `eal-cli` consumes clients, interfaces, and libs.
- Long-name repositories are compatibility history only. New package coordinates, issues, pull requests, releases, and submodule adoption use `eal-*`.

## Validation fleet

The separate `embedded-alerts-test` organization already exercises Flutter/web UI, API, WebSocket delivery, provider adapters, offline queues, embedded-host interop, accessibility, rate-limit retry, and MCP contract scenarios. The planned product-level `eal-e2e` repository consolidates cross-product smoke contracts without replacing that specialized validation fleet.

## Remaining repositories

| Repository | GitHub tracker | Linear tracker |
| --- | --- | --- |
| `embedded-alerts/eal-mcp-server.rs` | `eal-monorepo#2` | `DEN-2287` |
| `embedded-alerts/eal-e2e` | `eal-monorepo#7` | `DEN-2288` |

Both repositories are blocked only on organization-level repository creation. Once created, the connected GitHub write path can create branches and files, push commits, open pull requests, inspect checks, and merge.

## Git and Zed ownership rule

Git submodules remain valid exact-source transport, but the same repository must not be represented twice in one composition. Intentional Zed adoption uses `zed overtake --git-submodules`: Git retains the committed gitlink and source checkout, while Zed owns package identity, dependency intent, materialization, and immutable lock provenance. Non-Zed submodules remain solely Git-managed.

Every committed gitlink must be classified in `.zed-submodules.tsv`. CI rejects unclassified gitlinks, long-name duplicate coordinates, committed `.vendor/.zed` or `zed_modules` content, and any repository used simultaneously as a Zed dependency and a submodule.

## Planning authorities

- GitHub organization: `embedded-alerts`
- GitHub Project: organization Project #1
- Linear project: `github.com/embedded-alerts`
- Parent fleet issue: `DEN-1949`
- Repository-creation capability issue: `DEN-319`

GitHub issues and implementation pull requests must link the matching Linear issue and organization Project. Status is updated in both systems when a repository is created, a PR is merged, or a dependency/lock gate changes.