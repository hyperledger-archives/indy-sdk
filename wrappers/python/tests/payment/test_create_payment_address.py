from indy import IndyError
from indy import payment
from indy.error import ErrorCode
from tests.payment.constants import *
import pytest


@pytest.mark.asyncio
async def test_create_payment_address_works_for_unknown_payment_method(wallet_handle):
    with pytest.raises(IndyError) as e:
        await payment.create_payment_address(wallet_handle, payment_method, empty_object)
    assert ErrorCode.PaymentUnknownMethodError == e.value.error_code
