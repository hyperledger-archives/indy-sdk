from indy import IndyError
from indy import payment
from indy.error import ErrorCode
from tests.payment.constants import *

import pytest


@pytest.mark.asyncio
async def test_build_payment_request_works_for_unknown_payment_method(wallet_handle, did_trustee):
    with pytest.raises(IndyError) as e:
        await payment.build_payment_req(wallet_handle, did_trustee, inputs, outputs, None)
    assert ErrorCode.PaymentUnknownMethodError == e.value.error_code


@pytest.mark.asyncio
async def test_build_payment_request_works_for_empty_inputs(wallet_handle, did_trustee):
    with pytest.raises(IndyError) as e:
        await payment.build_payment_req(wallet_handle, did_trustee, empty_array, outputs, None)
    assert ErrorCode.CommonInvalidStructure == e.value.error_code


@pytest.mark.asyncio
async def test_build_payment_request_works_for_empty_outputs(wallet_handle, did_trustee):
    with pytest.raises(IndyError) as e:
        await payment.build_payment_req(wallet_handle, did_trustee, inputs, empty_object, None)
    assert ErrorCode.CommonInvalidStructure == e.value.error_code


@pytest.mark.asyncio
async def test_build_payment_request_works_for_incompatible_payment_methods(wallet_handle, did_trustee):
    with pytest.raises(IndyError) as e:
        await payment.build_payment_req(wallet_handle, did_trustee, incompatible_inputs, outputs, None)
    assert ErrorCode.PaymentIncompatibleMethodsError == e.value.error_code


@pytest.mark.asyncio
async def test_build_payment_request_works_for_invalid_input(wallet_handle, did_trustee):
    with pytest.raises(IndyError) as e:
        await payment.build_payment_req(wallet_handle, did_trustee, invalid_inputs, outputs, None)
    assert ErrorCode.CommonInvalidStructure == e.value.error_code