import pytest

from indy import payment, error
from tests.payment.constants import *


@pytest.mark.asyncio
async def test_build_get_txn_fees_request_works_for_unknown_payment_method(wallet_handle, did_trustee):
    with pytest.raises(error.PaymentUnknownMethodError):
        await payment.build_get_txn_fees_req(wallet_handle, did_trustee, payment_method)
