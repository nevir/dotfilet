# AI Agent Context

## Toolchain

- [Mise](http://mise.jdx.dev/) is used to manage the toolchain.
  - Arbitrary tools can be run via `mise exec -- COMMAND`

    Example: `mise exec -- dprint fmt AGENTS.md`

  - Common tasks are declared via Mise, and are run via `mise run TASK [ARGS…]`

    - Tasks can be discovered via `mise task ls`, and are defined in [.mise.toml](./.mise.toml)

## Behaviors

- After editing a file, ALWAYS run `mise fix-style FILE` to auto-format it.

## Helpful Tasks

- To view the merged output of an example: `mise example-config EXAMPLE-DIR-NAME`.

  Example: `mise example-config multi-host`
