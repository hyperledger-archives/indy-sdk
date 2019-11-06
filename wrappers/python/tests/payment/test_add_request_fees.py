import pytest

from indy import payment, error
from tests.payment.constants import *


@pytest.mark.asyncio
async def test_add_request_fees_works_for_unknown_payment_method(wallet_handle, did_trustee):
    with pytest.raises(error.PaymentUnknownMethodError):
        await payment.add_request_fees(wallet_handle, did_trustee, empty_object, inputs, empty_array, None)


@pytest.mark.asyncio
async def test_add_request_fees_works_for_empty_inputs(wallet_handle, did_trustee):
    with pytest.raises(error.CommonInvalidStructure):
        await payment.add_request_fees(wallet_handle, did_trustee, empty_object, '[]', empty_array, None)


@pytest.mark.asyncio
async def test_add_request_fees_works_for_several_methods(wallet_handle, did_trustee):
    with pytest.raises(error.PaymentIncompatibleMethodsError):
        await payment.add_request_fees(wallet_handle, did_trustee, empty_object, incompatible_inputs, empty_array, None)