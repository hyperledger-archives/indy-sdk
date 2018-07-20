from indy import IndyError
from indy import payment
from indy.error import ErrorCode
from tests.payment.constants import *

import pytest


@pytest.mark.asyncio
async def test_build_mint_request_works_for_unknown_payment_method(wallet_handle, did_trustee):
    with pytest.raises(IndyError) as e:
        await payment.build_mint_req(wallet_handle, did_trustee, outputs, None)
    assert ErrorCode.PaymentUnknownMethodError == e.value.error_code


@pytest.mark.asyncio
async def test_build_mint_request_works_for_empty_outputs(wallet_handle, did_trustee):
    with pytest.raises(IndyError) as e:
        await payment.build_mint_req(wallet_handle, did_trustee, empty_object, None)
    assert ErrorCode.CommonInvalidStructure == e.value.error_code


@pytest.mark.asyncio
async def test_build_mint_request_works_for_incompatible_outputs(wallet_handle, did_trustee):
    with pytest.raises(IndyError) as e:
        await payment.build_mint_req(wallet_handle, did_trustee, incompatible_outputs, None)
    assert ErrorCode.PaymentIncompatibleMethodsError == e.value.error_code