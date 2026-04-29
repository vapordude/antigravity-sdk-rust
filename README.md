# Google Antigravity SDK

The Google Antigravity SDK is a Python SDK for building AI agents powered by
Antigravity and Gemini. It provides a secure, scalable, and stateful
infrastructure layer that abstracts the agentic loop, letting you focus on what
your agent *does* rather than how it runs.

## Installation

```sh
pip install google-antigravity
```

## Quickstart

### Simple Agent

The `Agent` class is the easiest way to get started. It manages the full
lifecycle — binary discovery, tool wiring, hook registration, and policy
defaults — behind a single async context manager.

```python
import asyncio
from google.antigravity import Agent

async def main():
    async with Agent(api_key="GEMINI_API_KEY") as agent:
        response = await agent.chat("What files are in the current directory?")
        print(response)

asyncio.run(main())
```

By default, `Agent` runs in **read-only mode** for safety. Pass
`read_only=False` to enable write operations.

### Interactive Loop

```python
async with Agent(api_key="GEMINI_API_KEY", read_only=False) as agent:
    await agent.run_interactive_loop()
```

### Advanced Usage with Conversation

For full control over the connection lifecycle, use `Conversation` with a
`ConnectionStrategy` directly:

```python
import asyncio
from google.antigravity.connections.local_connection import LocalConnectionStrategy
from google.antigravity.conversation.conversation import Conversation
from google.antigravity.tools.tool_runner import ToolRunner
from google.antigravity.types import GeminiConfig

async def main():
    tool_runner = ToolRunner()
    strategy = LocalConnectionStrategy(
        binary_path="/path/to/localharness",
        tool_runner=tool_runner,
        gemini_config=GeminiConfig(api_key="GEMINI_API_KEY"),
    )
    conversation = await Conversation.create(strategy)

    await conversation.send("Hello!")
    async for step in conversation.receive_steps():
        if step.is_final_response:
            print(step.content)

    await strategy.stop()

asyncio.run(main())
```

## Features

### Custom Tools

Register Python functions as tools that the agent can call:

```python
def get_weather(city: str) -> str:
    """Returns the current weather for a city."""
    return f"It's sunny in {city}."

async with Agent(tools=[get_weather]) as agent:
    response = await agent.chat("What's the weather in Tokyo?")
```

### MCP Integration

Connect to external [MCP](https://modelcontextprotocol.io/) servers and expose
their tools to the agent:

```python
from google.antigravity import Agent

async with Agent(mcp_servers=["npx my-mcp-server"]) as agent:
    response = await agent.chat("Use the MCP tools to help me.")
```

### Hooks and Policies

Control agent behavior with a declarative policy system:

```python
from google.antigravity import Agent
from google.antigravity.hooks.policy import deny, allow, ask_user, enforce

policies = [
    deny("*"),                          # Block all tools by default
    allow("view_file"),                 # Allow reading files
    ask_user("run_command", handler=my_handler),  # Ask before running commands
]

async with Agent(policies=policies) as agent:
    await agent.run_interactive_loop()
```

### Triggers

Run background tasks that react to external events and push messages into the
agent:

```python
from google.antigravity import Agent
from google.antigravity.triggers import every

async def check_status(ctx):
    ctx.send("Check the deployment status.")

async with Agent(triggers=[every(60, check_status)]) as agent:
    await agent.run_interactive_loop()
```

## Architecture

The SDK follows a three-layer architecture:

| Layer | Purpose | Key Classes |
|:------|:--------|:------------|
| **Layer 1** — Simplified | High-level, easy-to-use entry point | `Agent`, `Conversation` |
| **Layer 2** — Core SDK | Full power for advanced users | `Step`, `ToolCall`, `HookRunner`, `ToolRunner`, `TriggerRunner` |
| **Layer 3** — Adapter | Transport and backend abstraction | `Connection`, `ConnectionStrategy`, `LocalConnection` |

## Examples

See the [`examples/`](examples/) directory for complete, runnable examples:

- **Local connection** — custom tools, MCP, and policy hooks with
  `LocalConnectionStrategy`
- **MCP server** — example MCP server with "pirate math" tools
- **Agent examples** — simple agent, custom tools + MCP, policies, hooks and
  triggers

## License

[Apache License 2.0](LICENSE)
