# Antigravity SDK Connections

The `connections` package provides the abstraction layer for interacting with agent backends. It decouples the higher-level SDK components (like `Conversation` and `Agent`) from the specific transport and process management details of where the agent is running.

## Core Abstractions

### `Connection`

`Connection` is an abstract base class that represents a live session with an agent backend. Layer 2 APIs depend ONLY on this interface.

Key methods and properties:
- `send(prompt, **kwargs)`: Sends a prompt to the agent.
- `receive_steps()`: An async iterator that yields `Step` objects as they are completed by the agent.
- `disconnect()`: Disconnects the session and releases resources.
- `cancel()`: Cancels the current turn.
- `is_idle`: Property indicating if the connection is idle.
- `conversation_id`: Property returning the conversation identifier.

### `ConnectionStrategy`

`ConnectionStrategy` is an abstract base class for establishing a `Connection`. It handles process management, transport setup, authentication, and health checking specific to a backend type.

Key methods:
- `connect()`: Returns the established `Connection`.
- `__aenter__()` and `__aexit__()`: Support for use as an async context manager to manage the backend lifecycle.

## Implementations

### `LocalConnection`

`LocalConnection` (and its corresponding `LocalConnectionStrategy`) connects to a Go-based local harness.

- **Transport**: It uses WebSockets to communicate with the Go harness.
- **Protocol**: It communicates using protobuf messages (`OutputEvent`, `InputEvent`, `StepUpdate`, etc.) serialized to JSON.
- **Features**:
    - Handles tool calls by executing them via `ToolRunner` and sending results back.
    - Handles question requests from the harness and dispatches them via `HookRunner` (interaction hooks).
    - Dispatches session start/end and turn hooks.

## Usage Example

```python
from google.antigravity.connections.local_connection import LocalConnectionStrategy
from google.antigravity.types import GeminiConfig

strategy = LocalConnectionStrategy(
    binary_path="/path/to/localharness",
    gemini_config=GeminiConfig(api_key="..."),
)

async with strategy as connection:
    await connection.send("Hello")
    async for step in connection.receive_steps():
        print(step)
```

## Files

- `connection.py`: Defines `Connection` and `ConnectionStrategy` interfaces.
- `local_connection.py`: Implements `LocalConnection` and `LocalConnectionStrategy`.
