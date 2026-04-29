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

import unittest
from unittest import mock

from google.antigravity import types
from google.antigravity.connections import connection

from google.antigravity.conversation import conversation


class ConversationTest(unittest.IsolatedAsyncioTestCase):
  """Validates the Conversation wrapper in conversation.py."""

  async def test_conversation_create(self):
    """Verifies that create delegates to strategy.connect."""
    mock_connection = mock.MagicMock(spec=connection.Connection)
    mock_strategy = mock.AsyncMock(spec=connection.ConnectionStrategy)
    mock_strategy.connect.return_value = mock_connection

    async with conversation.Conversation.create(mock_strategy) as conv:
      self.assertIsInstance(conv, conversation.Conversation)

    mock_strategy.connect.assert_called_once()

  async def test_send_delegation(self):
    """Verifies that send waits for idle and delegates to the underlying connection."""
    mock_connection = mock.AsyncMock(spec=connection.Connection)
    conv = conversation.Conversation(mock_connection)

    await conv.send("hello")

    mock_connection.wait_for_idle.assert_called_once()
    mock_connection.send.assert_called_once_with("hello")

  async def test_cancel_delegation(self):
    """Verifies that cancel delegates to the underlying connection."""
    mock_connection = mock.AsyncMock(spec=connection.Connection)
    conv = conversation.Conversation(mock_connection)

    await conv.cancel()

    mock_connection.cancel.assert_called_once()

  async def test_wait_for_idle_delegation(self):
    """Verifies that wait_for_idle delegates to the underlying connection."""
    mock_connection = mock.AsyncMock(spec=connection.Connection)
    conv = conversation.Conversation(mock_connection)

    await conv.wait_for_idle()

    mock_connection.wait_for_idle.assert_called_once()

  async def test_receive_steps_delegation(self):
    """Verifies that receive_steps yields from the underlying connection."""
    mock_connection = mock.MagicMock(spec=connection.Connection)

    async def mock_generator():
      yield types.Step(
          id="1",
          step_index=1,
          type=types.StepType.MODEL_RESPONSE,
          source=types.StepSource.MODEL,
          status=types.StepStatus.DONE,
          content="step1",
      )
      yield types.Step(
          id="2",
          step_index=2,
          type=types.StepType.MODEL_RESPONSE,
          source=types.StepSource.MODEL,
          status=types.StepStatus.DONE,
          content="step2",
      )

    mock_connection.receive_steps.return_value = mock_generator()
    conv = conversation.Conversation(mock_connection)

    steps = []
    async for step in conv.receive_steps():
      steps.append(step)

    self.assertEqual(len(steps), 2)
    self.assertEqual(steps[0].content, "step1")
    self.assertEqual(steps[1].content, "step2")

  async def test_disconnect_delegation(self):
    """Verifies that disconnect delegates to the underlying connection."""
    mock_connection = mock.AsyncMock(spec=connection.Connection)
    conv = conversation.Conversation(mock_connection)

    await conv.disconnect()

    mock_connection.disconnect.assert_called_once()


if __name__ == "__main__":
  unittest.main()
