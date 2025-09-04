# Configuration Schema

This document outlines the structure of the user-facing configuration and the internal compiled configuration for Dotfilet.

## File Structure

The recommended CUE file structure for a user's Dotfilet repository is as follows:

```
.
├─── dotfilet.cue  // Entrypoint
├─── hosts/
│   ├─── <hostname>.cue
│   └─── ...
└─── packages/
    ├─── <package-name>.cue
    └─── ...
```

- `dotfilet.cue`: The main entry point for the configuration. It imports and orchestrates the other files.
- `hosts/`: Contains host-specific configurations. Each file corresponds to a hostname.
- `packages/`: Contains configurations for different software packages (e.g., tools, applications).

## Top-Level Schema

The CUE configuration compiles to a single JSON object with the following top-level schema:

```json
{
  "hostname": "string",
  "packages": {
    "<package-name>": {
      "version": "string",
      "settings": {}
    }
  }
}
```

- `hostname`: The name of the host, used to select the correct host-specific configuration.
- `packages`: An object containing the configuration for each package.
  - `<package-name>`: The name of the package.
    - `version`: The version of the package to be installed.
    - `settings`: An object containing package-specific settings.

## Example

Here is an example of how a user would configure a simple tool, `git`, with a specific version and a setting to configure the user's email.

**`dotfilet.cue`**

```cue
import "hosts/my-laptop.cue"

#Config: {
  hostname: "my-laptop"
  packages: {
    git: {
      version: "2.39.1"
      settings: {
        email: "user@example.com"
      }
    }
  }
}
```

**`hosts/my-laptop.cue`**

```cue
#Host: {
  // Host-specific settings can be defined here
}
```

**`packages/git.cue`**

```cue
#Package: {
  name: "git"
  // Package-specific schema can be defined here
}
```
