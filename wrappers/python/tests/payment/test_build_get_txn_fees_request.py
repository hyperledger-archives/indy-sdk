from indy import IndyError
from indy import payment
from indy.error import ErrorCode
from tests.payment.constants import *

import pytest


@pytest.mark.asyncio
async def test_build_get_txn_fees_request_works_for_unknown_payment_method(wallet_handle, did_trustee):
    with pytest.raises(IndyError) as e:
        await payment.build_get_txn_fees_req(wallet_handle, did_trustee, payment_method)
    assert ErrorCode.PaymentUnknownMethodError == e.value.error_code
