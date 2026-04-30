# Antigravity SDK MCP Integration

The `mcp` package provides integration with the Model Context Protocol (MCP), allowing the agent to use tools exposed by external MCP servers.

## Core Concepts

### `McpBridge`

`McpBridge` simplifies the lifecycle of MCP client sessions and tool registration. It allows connecting to MCP servers and automatically registering their tools with the SDK's `ToolRunner`.

Supported connection types:
- **stdio**: For connecting to local MCP servers running as subprocesses.
- **SSE (Server-Sent Events)**: For connecting to remote MCP servers over HTTP.

### `register_mcp_tools`

A helper function that fetches tools from an MCP `ClientSessionGroup` and registers them as tools in a `ToolRunner`. It wraps MCP tools so they can be called like regular Python functions by the `ToolRunner`.

## Usage Example

```python
from google.antigravity.mcp.bridge import McpBridge
from google.antigravity.tools.tool_runner import ToolRunner

tool_runner = ToolRunner()
bridge = McpBridge(tool_runner)

# Connect to a local MCP server
await bridge.connect_stdio(
    command="npx",
    args=["-y", "@modelcontextprotocol/server-everything"]
)

# Tools from the MCP server are now registered in tool_runner
# and available to the agent.

# Cleanup when done
await bridge.stop()
```

## Files

- `bridge.py`: Defines `McpBridge` and `register_mcp_tools`.
