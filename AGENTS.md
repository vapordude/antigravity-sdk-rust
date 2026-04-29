# Contributing to Antigravity Python SDK

This document outlines the guidelines and best practices for developing and
testing the Antigravity Python SDK. Follow these patterns when making
incremental changes to ensure code quality and maintainability.

## Testing Principles

### 1. Strategic Mocking

Only mock at external service boundaries. For example:

-   `subprocess.Popen`
-   GRPC Stubs and Channel interactions
-   User input via `builtins.input`
-   External API and file system side-effects

Do **not** mock internal domain logic, classes (like `ToolRunner` or
`HookRunner`), or data transformations. We want the authentic logic of each
layer to run during tests.

### 2. Code Coverage

Always check coverage and ensure that you are at 100% coverage for the files you
are working on. To run tests with coverage enabled, execute:

```bash
bazel coverage //path/to/package/... --instrumentation_filter=//path/to/package
```

### 3. Explicit Branch Verification

Unit tests must cover structural code paths, not just the "happy path". Actively
verify:

- Timeout conditions and retry loops
- Error handling and translation of remote exceptions
- Validation of edge-case inputs (empty, malformed, or missing configs)

### 4. Strict File Segregation

Tests should be scaled 1:1 with source files where logical (e.g.,
`clients/types.py` should be tested by `clients/types_test.py` in the same
directory).

Specialized test suites (such as E2E or internal Google integration tests)
can be placed in a dedicated `tests/` subdirectory or use clear suffix
naming conventions.

### 5. Thorough Documentation and Docstrings

Ambiguity is unacceptable. Every test and non-trivial function must feature a
comprehensive docstring explaining:

- **What** logic is being verified or implemented.
- **Why** the verification is necessary (the business or architectural rationale).
- **How** the assertions validate the behavior.

### 6. Lint and Style Compliance

All code changes must be checked for style:

- Run `hg lint` on your workspace before submitting code to enforce standard
  Python formatting rules and Google coding standards.
