import pytest
from vcx.error import ErrorCode, VcxError, error_message
from vcx.api.wallet import Wallet
from vcx.common import update_institution_info
from vcx.api.connection import Connection
from vcx.api.utils import vcx_pool_set_handle
from vcx.api.vcx_init import  vcx_init_minimal


@pytest.mark.asyncio
@pytest.mark.usefixtures('vcx_init_test_mode')
async def test_vcx_init():
    pass


@pytest.mark.asyncio
async def test_error_message(vcx_init_test_mode):
    assert error_message(ErrorCode.NotReady) == 'Object not ready for specified action'


def test_vcx_init_minimal():
    Wallet.set_handle(1)
    vcx_pool_set_handle(1)
    assert vcx_init_minimal('''{"institution_name":"faber","institution_did":"44x8p4HubxzUK1dwxcc5FU",
                             "institution_verkey":"444MFrZjXDoi2Vc8Mm14Ys112tEZdDegBZZoembFEATE"}''') == 0
