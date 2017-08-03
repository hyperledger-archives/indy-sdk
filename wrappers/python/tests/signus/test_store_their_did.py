from indysdk import IndyError
from indysdk import signus

import pytest

from indysdk.error import ErrorCode


@pytest.mark.asyncio
async def test_store_their_did_works(wallet_handle):
    await signus.store_their_did(wallet_handle, '{"did":"8wZcEriaNLNKtteJvx7f8i"}')


@pytest.mark.asyncio
async def test_store_their_did_works_for_invalid_json(wallet_handle):
    with pytest.raises(IndyError) as e:
        await signus.store_their_did(wallet_handle, '{"field":"value"}')
    assert ErrorCode.CommonInvalidStructure == e.value.error_code


@pytest.mark.asyncio
async def test_store_their_did_works_for_invalid_handle(wallet_handle):
    with pytest.raises(IndyError) as e:
        await signus.store_their_did(wallet_handle + 1, '{"did":"8wZcEriaNLNKtteJvx7f8i"}')
    assert ErrorCode.WalletInvalidHandle == e.value.error_code


@pytest.mark.asyncio
async def test_store_their_did_works_with_verkey(wallet_handle):
    await signus.store_their_did(wallet_handle, '{"did":"8wZcEriaNLNKtteJvx7f8i",'
                                                ' "verkey": "GjZWsBLgZCR18aL468JAT7w9CZRiBnpxUPPgyQxh4voa"}')


@pytest.mark.asyncio
async def test_store_their_did_works_without_did(wallet_handle):
    with pytest.raises(IndyError) as e:
        await signus.store_their_did(wallet_handle, '{"verkey": "GjZWsBLgZCR18aL468JAT7w9CZRiBnpxUPPgyQxh4voa"}')
    assert ErrorCode.CommonInvalidStructure == e.value.error_code


@pytest.mark.asyncio
async def test_store_their_did_works_for_correct_crypto_type(wallet_handle):
    await signus.store_their_did(wallet_handle, '{"did":"8wZcEriaNLNKtteJvx7f8i", "crypto_type": "ed25519"}')


@pytest.mark.asyncio
async def test_store_their_did_works_for_invalid_did(wallet_handle):
    with pytest.raises(IndyError) as e:
        await signus.store_their_did(wallet_handle, '{"did": "invalid_base58_string"}')
    assert ErrorCode.CommonInvalidStructure == e.value.error_code

