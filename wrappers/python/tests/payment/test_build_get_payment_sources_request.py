from indy import IndyError
from indy import payment
from indy.error import ErrorCode
from tests.payment.constants import *

import pytest


@pytest.mark.asyncio
async def test_build_get_payment_sources_request_works_for_unknown_payment_method(wallet_handle, did_trustee):
    with pytest.raises(IndyError) as e:
        await payment.build_get_payment_sources_request(wallet_handle, did_trustee, payment_address)
    assert ErrorCode.PaymentUnknownMethodError == e.value.error_code


@pytest.mark.asyncio
async def test_build_get_payment_sources_request_works_for_invalid_payment_address(wallet_handle, did_trustee):
    with pytest.raises(IndyError) as e:
        await payment.build_get_payment_sources_request(wallet_handle, did_trustee, "pay:null1")
    assert ErrorCode.PaymentIncompatibleMethodsError == e.value.error_code