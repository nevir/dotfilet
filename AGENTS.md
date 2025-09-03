# AI Agent Context

## Toolchain

- [Mise](http://mise.jdx.dev/) is used to manage the toolchain.
  - Arbitrary tools can be run via `mise exec -- COMMAND`
    Example: `mise exec -- dprint fmt AGENTS.md`
  - Common tasks are exposed via mise tasks (configured in [.mise.toml](./.mise.toml)) and can be run via `mise [run] TASK [ARGS…]`

## Behaviors

- After editing a file, ALWAYS run `mise fix-style FILE` to auto-format it.
