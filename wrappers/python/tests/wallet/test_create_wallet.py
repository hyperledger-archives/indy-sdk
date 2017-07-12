from indy import Wallet

from ..utils.storage import StorageUtils
from ..utils.logging import logger

import pytest


@pytest.mark.asyncio
async def test_indy_create_wallet_works():
    logger.debug("asasd")
    StorageUtils.cleanup()

    pool_name = 'indy_create_wallet_works'
    wallet_name = 'indy_create_wallet_works'
    xtype = 'default'

    await Wallet.create_wallet(pool_name, wallet_name, xtype, None, None)

    StorageUtils.cleanup()