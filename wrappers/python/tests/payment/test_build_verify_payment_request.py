import pytest

from indy import payment, error
from tests.payment.constants import *


@pytest.mark.asyncio
async def test_build_verify_payment_request_works_for_unknown_payment_method(wallet_handle, did_trustee):
    with pytest.raises(error.PaymentUnknownMethodError):
        await payment.build_verify_payment_req(wallet_handle, did_trustee, receipt)
