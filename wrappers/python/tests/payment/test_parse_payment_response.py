from indy import IndyError
from indy import payment
from indy.error import ErrorCode
from tests.payment.constants import *

import pytest


@pytest.mark.asyncio
async def test_parse_payment_response_works_for_unknown_payment_method():
    with pytest.raises(IndyError) as e:
        await payment.parse_payment_response(payment_method, empty_object)
    assert ErrorCode.PaymentUnknownMethodError == e.value.error_code
