import pytest

from indy import payment, error
from tests.payment.constants import *


@pytest.mark.asyncio
async def test_create_payment_address_works_for_unknown_payment_method(wallet_handle):
    with pytest.raises(error.PaymentUnknownMethodError):
        await payment.create_payment_address(wallet_handle, payment_method, empty_object)
