from indy import IndyError
from indy import payment
from indy.error import ErrorCode
from tests.payment.constants import *

import pytest


@pytest.mark.asyncio
async def test_build_set_txn_fees_request_works_for_unknown_payment_method(wallet_handle, did_trustee):
    with pytest.raises(IndyError) as e:
        await payment.build_set_txn_fees_req(wallet_handle, did_trustee, payment_method, fees)
    assert ErrorCode.PaymentUnknownMethodError == e.value.error_code


@pytest.mark.asyncio
async def test_build_set_txn_fees_request_works_for_invalid_fees(wallet_handle, did_trustee):
    with pytest.raises(IndyError) as e:
        await payment.build_set_txn_fees_req(wallet_handle, did_trustee, payment_method, '[txnType1:1, txnType2:2]')
    assert ErrorCode.CommonInvalidStructure == e.value.error_code