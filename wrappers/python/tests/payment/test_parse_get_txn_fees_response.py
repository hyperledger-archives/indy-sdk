import pytest

from indy import payment, error
from tests.payment.constants import *


@pytest.mark.asyncio
async def test_parse_get_txn_fees_response_works_for_unknown_payment_method():
    with pytest.raises(error.PaymentUnknownMethodError):
        await payment.parse_get_txn_fees_response(payment_method, empty_object)
