import pytest
from vcx.api.vcx_init import vcx_init


@pytest.mark.asyncio
@pytest.fixture
async def vcx_init_test_mode():
    if not hasattr(vcx_init, "open"):
        vcx_init.open = None

    if not vcx_init.open:
        await vcx_init('ENABLE_TEST_MODE')
        vcx_init.open = True

