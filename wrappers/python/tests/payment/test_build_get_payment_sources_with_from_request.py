import pytest

from indy import payment, error
from tests.payment.constants import *


@pytest.mark.asyncio
async def test_build_get_payment_sources_with_from_request_works_for_unknown_payment_method(wallet_handle, did_trustee):
    with pytest.raises(error.PaymentUnknownMethodError):
        await payment.build_get_payment_sources_with_from_request(wallet_handle, did_trustee, payment_address, from_seqno=1)


@pytest.mark.asyncio
async def test_build_get_payment_sources_with_from_request_works_for_unknown_payment_method_no_from_seqno(wallet_handle, did_trustee):
    with pytest.raises(error.PaymentUnknownMethodError):
        await payment.build_get_payment_sources_with_from_request(wallet_handle, did_trustee, payment_address)


@pytest.mark.asyncio
async def test_build_get_payment_sources_with_from_request_works_for_invalid_payment_address(wallet_handle, did_trustee):
    with pytest.raises(error.PaymentIncompatibleMethodsError):
        await payment.build_get_payment_sources_with_from_request(wallet_handle, did_trustee, "pay:null1")
