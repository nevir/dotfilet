# AI Agent Context

## Planning

- Work is planned via GitHub Issues. Prefer interacting with them via the GitHub MCP server.

- Issues should represent atomic pieces of work (roughly equivalent to a small pull request).

  - If they need to represent a larger project, break the work down into sub-issues (using GitHub's sub-issue API).

- Every issue's body should clearly describe the desired outcome, motivation and requirements.

- Use issue comments to take notes and work through intermediate progress or ideas.

## Toolchain

- [Mise](http://mise.jdx.dev/) is used to manage the toolchain.
  - Arbitrary tools can be run via `mise exec -- COMMAND`

    Example: `mise exec -- dprint fmt AGENTS.md`

  - Common tasks are declared via Mise, and are run via `mise run TASK [ARGS…]`

    - Tasks can be discovered via `mise task ls`, and are defined in [.mise.toml](./.mise.toml)

## Behaviors

- After editing a file, ALWAYS run `mise run fix-style FILE` to auto-format it.

## Helpful Tasks

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
