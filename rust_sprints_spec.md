# Google Antigravity SDK: Rust Migration Sprint Specification

**Goal:** Port `google-antigravity` from Python to Rust, ensuring functional parity, performance improvements, and full integration with existing testing and deployment infrastructure.

## Sprint 1: Project Setup and Core Types Translation
*   **Objective:** Set up the Rust repository structure, CI/CD, and define the foundational types and models.
*   **Tasks:**
    *   Initialize a new Cargo workspace for the Rust SDK.
    *   Set up CI (GitHub Actions or similar) with `rustfmt`, `clippy`, and basic test runners.
    *   Translate `google/antigravity/types.py` into Rust structs and enums (`src/types.rs` or a separate `types` crate).
    *   Ensure accurate deserialization/serialization with `serde` and `serde_json`, matching the Python payload structures.
    *   Set up basic unit testing for serialization/deserialization.
    *   Implement equivalent of `google-genai` types if necessary or find a suitable Rust crate for Gemini interactions.

## Sprint 2: Connections and Local Harness Integration
*   **Objective:** Implement the lowest layer (Layer 3 - Transport and Backend Abstraction), focusing on the `LocalConnection` and communicating with the `localharness` binary.
*   **Tasks:**
    *   Translate the `Connection` trait/interface from `google/antigravity/connections/connection.py`.
    *   Implement `LocalConnection` (`src/connections/local.rs`) using `tokio` for async I/O.
    *   Port the protobuf definitions (`localharness.proto` -> `localharness.rs`) using `prost` or `tonic-build`.
    *   Implement the subprocess management to spawn and communicate with the Go `localharness` binary.
    *   Implement WebSocket or Standard IO communication with the harness.
    *   Create integration tests to verify the Rust `LocalConnection` can start and communicate with a real `localharness` binary.

## Sprint 3: Tools, Triggers, and Hooks (Layer 2)
*   **Objective:** Implement the intermediate state and lifecycle management components.
*   **Tasks:**
    *   Implement `ToolRunner` and tool registration mechanisms (`src/tools.rs`). Since Rust is statically typed, this will require defining a macro or trait system for registering Rust functions as tools.
    *   Implement `HookRunner` and policy enforcement (`src/hooks.rs`). Translate `allow`, `deny`, `ask_user` logic.
    *   Implement `TriggerRunner` for background tasks (`src/triggers.rs`).
    *   Develop comprehensive unit tests for tool routing, hook execution order, and trigger scheduling.

## Sprint 4: Conversation Session and State Management
*   **Objective:** Implement the `Conversation` class (Layer 2) that manages step history and turn state.
*   **Tasks:**
    *   Implement the `Conversation` struct (`src/conversation.rs`).
    *   Implement history accumulation, token tracking, and state introspection.
    *   Implement the `chat()` convenience method, handling the stream of steps from the `Connection`.
    *   Write integration tests simulating multi-turn conversations using mock connections.

## Sprint 5: The Agent Abstraction and MCP (Layer 1)
*   **Objective:** Implement the high-level `Agent` entry point and integrate MCP.
*   **Tasks:**
    *   Implement the `Agent` struct (`src/agent.rs`), wrapping the `Conversation` and configuration.
    *   Implement `AgentConfig`, `CapabilitiesConfig`, etc.
    *   Integrate Model Context Protocol (MCP) clients in Rust to match the `McpStdioServer` functionality.
    *   Ensure the streaming API is ergonomic (e.g., using `futures::Stream`).

## Sprint 6: Examples, Documentation, and Interactive Loop
*   **Objective:** Polish the SDK, provide examples, and build the interactive CLI loop.
*   **Tasks:**
    *   Translate `google/antigravity/utils/interactive.py` into a Rust CLI utility using `tokio` and a terminal library like `crossterm` or `rustyline`.
    *   Port the examples from `examples/` to Rust.
    *   Write comprehensive documentation (rustdoc) for all public APIs.
    *   Update `README.md` to reflect Rust usage.

## Sprint 7: End-to-End Verification and Release Polish
*   **Objective:** Ensure the Rust SDK perfectly mirrors the Python SDK's capabilities and is ready for release.
*   **Tasks:**
    *   Write end-to-end black-box tests that run the same prompts through both the Python and Rust SDKs and assert equivalent outputs and tool call sequences.
    *   Performance profiling and optimization (e.g., reducing allocations, optimizing serialization).
    *   Finalize packaging and publishing scripts (e.g., publishing to crates.io, handling the `localharness` binary distribution in a Rust-idiomatic way).
