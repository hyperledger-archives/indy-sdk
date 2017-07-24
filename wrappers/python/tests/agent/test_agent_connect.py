import json
import logging

import pytest

from indy import signus, agent
from ..utils import storage, wallet


@pytest.mark.asyncio
async def test_agent_connect_works(connection):
    assert connection is not None
