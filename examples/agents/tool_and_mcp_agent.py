# Copyright 2026 Google LLC
#
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
#     https://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.

"""Agent example with custom tool and MCP server."""

import asyncio
import logging
import os
import tempfile
from google.antigravity.agent import Agent
from google.antigravity.examples import example_policies


def read_file_upside_down(path: str) -> str:
  """Reads the file at the given path and returns its content with lines inverted."""
  with open(path, "r") as f:
    lines = f.readlines()
  return "".join(reversed(lines))


async def main():
  logging.basicConfig(level=logging.INFO)

  # Find the MCP server binary relative to this script
  current_dir = os.path.dirname(os.path.abspath(__file__))
  mcp_server_path = os.path.abspath(
      os.path.join(current_dir, "..", "mcp_server.par")
  )

  if not os.path.exists(mcp_server_path):
    logging.warning("Failed to find mcp_server.par at %s", mcp_server_path)
    mcp_server_path = None

  mcp_servers = []
  if mcp_server_path:
    mcp_servers.append({
        "type": "stdio",
        "command": mcp_server_path,
        "args": ["--transport=stdio"],
    })

  print("Creating agent...")
  async with Agent(
      system_instructions=(
          "You are a helpful assistant. Use your tools when needed."
      ),
      tools=[read_file_upside_down],
      policies=[example_policies.BLOCK_RM_POLICY],
      mcp_servers=mcp_servers,
      read_only=False,  # We want to allow tools
  ) as agent:

    print("\nChatting with agent...")
    # Create a temp file to read
    with tempfile.NamedTemporaryFile(mode="w", delete=False) as f:
      f.write("Line 1\nLine 2\n")
      temp_path = f.name

    try:
      response = await agent.chat(f"Read the file at {temp_path} upside down.")
      print(f"Agent: {response.text}\n")
    finally:
      os.unlink(temp_path)


if __name__ == "__main__":
  asyncio.run(main())
