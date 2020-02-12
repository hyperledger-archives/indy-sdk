from indy import payment, error
from tests.payment.constants import *

import pytest


@pytest.mark.asyncio
async def test_parse_get_payment_sources_with_from_response_works_for_unknown_payment_method():
    with pytest.raises(error.PaymentUnknownMethodError):
        await payment.parse_get_payment_sources_with_from_response(payment_method, empty_object)
