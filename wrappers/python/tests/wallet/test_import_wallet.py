import pytest

from indy import did, error, wallet


@pytest.mark.asyncio
@pytest.mark.parametrize("wallet_handle_cleanup", [False])
async def test_import_wallet_works(wallet_handle, wallet_config, credentials, export_config):
    (_did, _verkey) = await did.create_and_store_my_did(wallet_handle, "{}")
    await did.set_did_metadata(wallet_handle, _did, "metadata")

    did_with_meta_before = await did.get_my_did_with_meta(wallet_handle, _did)

    await wallet.export_wallet(wallet_handle, export_config)

    await wallet.close_wallet(wallet_handle)
    await wallet.delete_wallet(wallet_config, credentials)

    await wallet.import_wallet(wallet_config, credentials, export_config)
    wallet_handle = await wallet.open_wallet(wallet_config, credentials)

    did_with_meta_after = await did.get_my_did_with_meta(wallet_handle, _did)
    assert did_with_meta_before == did_with_meta_after

    await wallet.close_wallet(wallet_handle)


@pytest.mark.asyncio
async def test_import_wallet_works_for_not_exit_path(wallet_config, credentials, export_config):
    with pytest.raises(error.CommonIOError):
        await wallet.import_wallet(wallet_config, credentials, export_config)
