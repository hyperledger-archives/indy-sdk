import pytest

from indy import payment, error
from tests.payment.constants import *


@pytest.mark.asyncio
async def test_verify_with_address_works_for_unknown_payment_method(message):
    signature = bytes([169, 215, 8, 225, 7, 107, 110, 9, 193, 162, 202, 214])
    with pytest.raises(error.PaymentUnknownMethodError):
        await payment.verify_with_address(payment_address, message, signature)
