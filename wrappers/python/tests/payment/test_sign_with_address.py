import pytest

from indy import payment, error
from tests.payment.constants import *


@pytest.mark.asyncio
async def test_sign_with_address_works_for_unknown_payment_method(wallet_handle, message):
    with pytest.raises(error.PaymentUnknownMethodError):
        await payment.sign_with_address(wallet_handle, payment_address, message)
