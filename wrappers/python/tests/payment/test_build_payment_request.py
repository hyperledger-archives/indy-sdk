import pytest

from indy import payment, error
from tests.payment.constants import *


@pytest.mark.asyncio
async def test_build_payment_request_works_for_unknown_payment_method(wallet_handle, did_trustee):
    with pytest.raises(error.PaymentUnknownMethodError):
        await payment.build_payment_req(wallet_handle, did_trustee, inputs, outputs, None)


@pytest.mark.asyncio
async def test_build_payment_request_works_for_empty_inputs(wallet_handle, did_trustee):
    with pytest.raises(error.CommonInvalidStructure):
        await payment.build_payment_req(wallet_handle, did_trustee, empty_array, outputs, None)


@pytest.mark.asyncio
async def test_build_payment_request_works_for_empty_outputs(wallet_handle, did_trustee):
    with pytest.raises(error.CommonInvalidStructure):
        await payment.build_payment_req(wallet_handle, did_trustee, inputs, empty_object, None)


@pytest.mark.asyncio
async def test_build_payment_request_works_for_incompatible_payment_methods(wallet_handle, did_trustee):
    with pytest.raises(error.PaymentIncompatibleMethodsError):
        await payment.build_payment_req(wallet_handle, did_trustee, incompatible_inputs, outputs, None)


@pytest.mark.asyncio
async def test_build_payment_request_works_for_invalid_input(wallet_handle, did_trustee):
    with pytest.raises(error.CommonInvalidStructure):
        await payment.build_payment_req(wallet_handle, did_trustee, invalid_inputs, outputs, None)
