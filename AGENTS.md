# AI Agent Context

> [!IMPORTANT]
>
> **CRITICAL: NO FILE SYSTEM MODIFICATIONS BEFORE BRANCHING**
>
> You MUST create a feature branch BEFORE making any changes to files, creating directories, or modifying the file system in any way. Read the [Development Workflow](#development-workflow) below and follow it exactly.

## Development Workflow

> [!WARNING]
>
> This workflow MUST be followed for every task. Failure to follow this workflow, especially creating a branch before making changes, will result in rejected modifications.

### MANDATORY Steps (In Order):

1. **📖 Read & Understand**: Thoroughly review the GitHub issue and all comments first.

   - If it's a sub-issue, review the parent issue and related sub-tasks.
   - **NO file system modifications during this step.**

2. **🌿 Create Branch**: Create a new feature branch BEFORE any file changes.

   ```bash
   git checkout -b feat/descriptive-name
   ```

   - Use kebab-case names with prefixes: `feat/`, `fix/`, `refactor/`, etc.
   - **This MUST happen before creating files, directories, or editing _anything_.**

3. **⚡ Implementation**: Only now perform code modifications, file creation, etc.

   - Group related changes into logical commits.
   - Follow existing code conventions and style.

4. **✅ Verification**: Test and lint before concluding work.

   - Run tests and linters to ensure quality.
   - Verify all issue requirements are met.
   - Create pull request when ready for review.

## Overview

This project, `dotfilet`, is a declarative configuration management tool for developer environments, initially targeting macOS. It treats your machine's configuration as code, enabling versioning, sharing, and reliable reproduction.

Key characteristics:

- **Declarative**: Define the desired state in CUE configuration files, and `dotfilet` handles the rest.

- **Bi-directional Sync**: Uniquely, it can read manual changes from the system and help you commit them back to your configuration.

- **Extensible**: A plugin system allows for managing various applications and system settings.

For a deeper understanding of the project's design, architecture, and concepts, please refer to the documentation in the [`docs`](./docs/) directory.

## Keeping this Document Updated

This document is a living guide for AI agents working on this project. To ensure future agents can work as efficiently and effectively as possible, it is crucial to keep this document current.

As you work, you will learn new things about the codebase, the tools, and the project's conventions. Please help improve this guide by:

- **Documenting Tool Quirks**: If you discover a non-obvious behavior or a specific way a tool needs to be used, add it to the relevant section (e.g., `Toolchain`).

- **Capturing Implicit Conventions**: If you notice a recurring style or pattern in the code that isn't explicitly documented, add it to the `Style` section or create a new one.

- **Correcting Outdated Information**: If you find that any part of this document is inaccurate or no longer relevant, please correct it immediately.

- **Adding Helpful Information**: If you find yourself repeatedly discovering the same information, consider adding it to this guide to help future agents. This could be a new helpful command, a common workflow, or a link to a useful resource.

Think of this as contributing to the collective knowledge of the team. Your updates will help all future agents get up to speed faster and avoid common pitfalls.

## Issue Tracking

- **Tracked in GitHub**: All work is tracked via GitHub Issues.
  - Use sub-issues to break down larger pieces of work.

- **Issue Templates**: Use an appropriate [issue template]((.github/ISSUE_TEMPLATE) (such as the [task template](.github/ISSUE_TEMPLATE/task.md)) to ensure all necessary information is included.

- **Collaboration**: Use issue comments for notes, ideas, and discussing progress.

## Toolchain

- [Mise](http://mise.jdx.dev/) is used to manage the toolchain.
  - Arbitrary tools can be run via `mise exec -- COMMAND`

    Example: `mise exec -- dprint fmt AGENTS.md`

  - Common tasks are declared via Mise, and are run via `mise run TASK [ARGS…]`

    - Tasks can be discovered via `mise task ls`, and are defined in [.mise.toml](./.mise.toml)

### Important Tasks

- After editing a file, ALWAYS run `mise run fix-style FILE` to auto-format it.

- To view the merged output of an example: `mise run example-config EXAMPLE-DIR-NAME`.

  Example: `mise run example-config multi-host`

## Style

### Markdown Style

- Write lists using the `-` character.

- Leave a newline between each list item, unless the list is enumerating short items (names, etc).

- **Labeled Lists**: When list items begin with a label, **bold** the text, but leave the `:` outside of the label.

- Use markdown alerts for "asides" and other secondary information. There must always be a blank `>` line between the alert type and body

  > [!NOTE]
  >
  > Supported alert types:
  >
  > - NOTE
  > - TIP
  > - IMPORTANT
  > - WARNING
  > - CAUTION
