# Plugin JSON-RPC Protocol

This document defines the communication protocol between the Dotfilet core system and plugins.

## Overview

Plugins are external processes that communicate with Dotfilet via JSON-RPC. Each plugin is responsible for managing a subset of the overall system configuration (e.g., VS Code settings, macOS Dock configuration, Git configuration).

## Transport

- **Primary**: JSON-RPC over WebSocket
- **Alternative**: Synchronous JSON-RPC over stdin/stdout (TBD)

The protocol should be documented similarly to the [Language Server Protocol (LSP)](https://microsoft.github.io/language-server-protocol/specifications/lsp/3.17/specification/).

## Methods

### Server → Plugin

#### `describeConfig`

Reports all known supported configuration values and their ranges.

**Request:**

```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "describeConfig",
  "params": {}
}
```

**Response:**

```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "result": {
    "schema": {
      // JSON Schema or CUE schema describing supported configuration
    }
  }
}
```

#### `getConfig`

Retrieves the current (remote) configuration from the system.

**Request:**

```json
{
  "jsonrpc": "2.0",
  "id": 2,
  "method": "getConfig",
  "params": {
    "keys": ["optional", "array", "of", "specific.config.keys"]
  }
}
```

**Response:**

```json
{
  "jsonrpc": "2.0",
  "id": 2,
  "result": {
    "config": {
      // Current configuration values
    }
  }
}
```

#### `applyConfig`

Idempotently applies the requested configuration to the system.

**Request:**

```json
{
  "jsonrpc": "2.0",
  "id": 3,
  "method": "applyConfig",
  "params": {
    "config": {
      // Configuration to apply
    }
  }
}
```

**Response:**

```json
{
  "jsonrpc": "2.0",
  "id": 3,
  "result": {
    "success": true,
    "changes": [
      // List of changes that were made
    ]
  }
}
```

#### `subscribeToRemoteChanges`

Requests that the plugin watch for changes and report them back via `remoteChanges` notifications.

**Request:**

```json
{
  "jsonrpc": "2.0",
  "id": 4,
  "method": "subscribeToRemoteChanges",
  "params": {
    "keys": ["optional", "array", "of", "keys.to.watch"]
  }
}
```

**Response:**

```json
{
  "jsonrpc": "2.0",
  "id": 4,
  "result": {
    "subscribed": true
  }
}
```

#### `fileSystemChanges`

Reports any files that have changed and the matching expressions that were subscribed to.

**Request:**

```json
{
  "jsonrpc": "2.0",
  "id": 5,
  "method": "fileSystemChanges",
  "params": {
    "changes": [
      {
        "path": "/path/to/changed/file",
        "type": "modified|created|deleted",
        "matchedExpressions": ["glob patterns that matched"]
      }
    ]
  }
}
```

### Plugin → Server

#### `remoteChanges` (Notification)

Sends any configuration values that were changed out of band (e.g., via GUI or other tools).

**Notification:**

```json
{
  "jsonrpc": "2.0",
  "method": "remoteChanges",
  "params": {
    "changes": {
      // Configuration keys and their new values
    }
  }
}
```

#### `subscribeToFilesystem`

Asks the server to perform file system watching on specific paths.

**Request:**

```json
{
  "jsonrpc": "2.0",
  "id": 6,
  "method": "subscribeToFilesystem",
  "params": {
    "patterns": [
      "~/Library/Preferences/com.example.app.plist",
      "~/.config/app/**/*.json"
    ]
  }
}
```

**Response:**

```json
{
  "jsonrpc": "2.0",
  "id": 6,
  "result": {
    "subscribed": true
  }
}
```

## Plugin Lifecycle

1. **Discovery**: Dotfilet discovers plugins via configuration or plugin registry
2. **Launch**: Plugin process is started
3. **Handshake**: Initial connection and capability negotiation
4. **Configuration**: Plugin describes its supported configuration schema
5. **Operation**: Normal read/write operations and change monitoring
6. **Shutdown**: Graceful termination

## Error Handling

Standard JSON-RPC error responses should be used:

```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "error": {
    "code": -32000,
    "message": "Plugin-specific error",
    "data": {
      // Additional error context
    }
  }
}
```

## Security Considerations

- Plugins should run in isolated processes
- File system access should be limited to declared paths
- Network access should be restricted as needed
- Configuration changes should be validated before application

## Future Considerations

- Plugin versioning and compatibility
- Bulk operations for performance
- Streaming for large configuration sets
- Plugin authentication and authorization
