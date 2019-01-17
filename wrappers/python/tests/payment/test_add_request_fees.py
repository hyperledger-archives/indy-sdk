from indy import IndyError
from indy import payment
from indy.error import ErrorCode
from tests.payment.constants import *

import pytest


@pytest.mark.asyncio
async def test_add_request_fees_works_for_unknown_payment_method(wallet_handle, did_trustee):
    with pytest.raises(IndyError) as e:
        await payment.add_request_fees(wallet_handle, did_trustee, empty_object, inputs, empty_array, None)
    assert ErrorCode.PaymentUnknownMethodError == e.value.error_code


@pytest.mark.asyncio
async def test_add_request_fees_works_for_empty_inputs(wallet_handle, did_trustee):
    with pytest.raises(IndyError) as e:
        await payment.add_request_fees(wallet_handle, did_trustee, empty_object, '[]', empty_array, None)
    assert ErrorCode.CommonInvalidStructure == e.value.error_code


@pytest.mark.asyncio
async def test_add_request_fees_works_for_several_methods(wallet_handle, did_trustee):
    with pytest.raises(IndyError) as e:
        await payment.add_request_fees(wallet_handle, did_trustee, empty_object, incompatible_inputs, empty_array, None)
    assert ErrorCode.PaymentIncompatibleMethodsError == e.value.error_code


@pytest.mark.asyncio
async def test_add_request_fees_works_for_invalid_input(wallet_handle, did_trustee):
    with pytest.raises(IndyError) as e:
        await payment.add_request_fees(wallet_handle, did_trustee, empty_object, invalid_inputs, empty_array, None)
    assert ErrorCode.CommonInvalidStructure == e.value.error_code
