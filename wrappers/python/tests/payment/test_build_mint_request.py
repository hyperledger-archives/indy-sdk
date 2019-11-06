import pytest

from indy import payment, error
from tests.payment.constants import *


@pytest.mark.asyncio
async def test_build_mint_request_works_for_unknown_payment_method(wallet_handle, did_trustee):
    with pytest.raises(error.PaymentUnknownMethodError):
        await payment.build_mint_req(wallet_handle, did_trustee, outputs, None)


@pytest.mark.asyncio
async def test_build_mint_request_works_for_empty_outputs(wallet_handle, did_trustee):
    with pytest.raises(error.CommonInvalidStructure):
        await payment.build_mint_req(wallet_handle, did_trustee, empty_object, None)


@pytest.mark.asyncio
async def test_build_mint_request_works_for_incompatible_outputs(wallet_handle, did_trustee):
    with pytest.raises(error.PaymentIncompatibleMethodsError):
        await payment.build_mint_req(wallet_handle, did_trustee, incompatible_outputs, None)
