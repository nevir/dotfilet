# AI Agent Context

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

## Workflow

This section describes the preferred workflow for contributing to this project.

### Issue Tracking

- **Tracked in GitHub**: All work is tracked via GitHub Issues.
  - Use sub-issues to break down larger pieces of work.

- **Atomic Tasks**: Each leaf issue should represent a small, atomic piece of work.
  - Aim for pull requests with around 300 changed lines or less, though this is not a strict rule.

- **Clear Description**: Every issue's body should clearly describe the desired outcome, motivation, and requirements.

- **Collaboration**: Use issue comments for notes, ideas, and discussing progress.

### Development

- **Understand the Task**: Before you begin, thoroughly review the issue and all of its comments.
  - If it is a sub-issue, review the parent issue and its other sub-tasks to ensure your work is correctly scoped and aligned with the overall goal.

- **Branching**: All work should be done within a feature branch.
  - **Branch Naming**: Use a descriptive, kebab-case name prefixed with `feat/`. For example: `feat/1pass-secrets`.

- **Commits**: Group related changes into logical commits. This repository uses a merge commit strategy, so individual commits within a branch are important.

- **Pull Requests**: When a feature is ready for review, push the branch and open a pull request.

- **Dependent Changes**: For tasks that depend on each other (e.g., an issue with multiple sub-issues), create a chain of branches.
  - Each branch should be based on the previous one.
  - The pull requests should also reflect this dependency chain (e.g., PR for `feat/b` targets `feat/a`).

- **Verification**: Before concluding your work on a task, double-check your changes against the issue's requirements and goals. Ensure you have fully addressed the task.

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
