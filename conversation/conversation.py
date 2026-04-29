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

"""Conversation wrapper for Antigravity SDK."""

import contextlib
from typing import Any, AsyncIterator

from google.antigravity import types
from google.antigravity.connections import connection


class Conversation:
  """Wrapper around a single conversation with the agent."""

  def __init__(
      self,
      conn: connection.Connection,
  ):
    """Initializes the conversation wrapper with a connection strategy."""
    self._connection = conn

  @classmethod
  @contextlib.asynccontextmanager
  async def create(
      cls,
      strategy: connection.ConnectionStrategy,
  ) -> AsyncIterator["Conversation"]:
    """Creates a new conversation.

    Args:
      strategy: The connection strategy to use to interact with an agent.
    """
    async with strategy:
      yield cls(strategy.connect())

  async def send(
      self,
      prompt: str | None,
      **kwargs: Any,
  ) -> None:
    """Sends a message to the agent."""
    await self.wait_for_idle()
    await self._connection.send(prompt, **kwargs)

  async def receive_steps(self) -> AsyncIterator[types.Step]:
    """Receives steps as they complete, blocks until execution is idle.

    Yields steps from the underlying connection. The iterator exits once
    the execution turn is complete.

    Yields:
      Steps as they complete.
    """
    async for step in self._connection.receive_steps():
      yield step

  async def cancel(self) -> None:
    """Cancels the current turn."""
    if hasattr(self._connection, "cancel"):
      await self._connection.cancel()

  async def delete(self) -> None:
    """Deletes this conversation from the backend."""
    if hasattr(self._connection, "delete"):
      await self._connection.delete()

  async def signal_idle(self) -> None:
    """Signals that the conversation is ready to receive input."""
    if hasattr(self._connection, "signal_idle"):
      await self._connection.signal_idle()

  async def wait_for_idle(self) -> None:
    """Blocks until the conversation is idle."""
    if hasattr(self._connection, "wait_for_idle"):
      await self._connection.wait_for_idle()

  async def wait_for_wakeup(self, timeout: float = 300.0) -> bool:
    """Blocks until the conversation wakes up."""
    if hasattr(self._connection, "wait_for_wakeup"):
      return await self._connection.wait_for_wakeup(timeout)
    return False

  async def disconnect(self) -> None:
    """Disconnect the conversation's background stream."""
    if hasattr(self._connection, "disconnect"):
      await self._connection.disconnect()

  @property
  def conversation_id(self) -> str:
    """Returns the ID of the background conversation if one exists."""
    if hasattr(self._connection, "_conversation_id"):
      return self._connection._conversation_id  # pylint: disable=protected-access
    return ""
