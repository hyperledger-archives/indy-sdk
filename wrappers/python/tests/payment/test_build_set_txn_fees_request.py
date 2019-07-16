import pytest

from indy import payment, error
from tests.payment.constants import *


@pytest.mark.asyncio
async def test_build_set_txn_fees_request_works_for_unknown_payment_method(wallet_handle, did_trustee):
    with pytest.raises(error.PaymentUnknownMethodError):
        await payment.build_set_txn_fees_req(wallet_handle, did_trustee, payment_method, fees)


@pytest.mark.asyncio
async def test_build_set_txn_fees_request_works_for_invalid_fees(wallet_handle, did_trustee):
    with pytest.raises(error.CommonInvalidStructure):
        await payment.build_set_txn_fees_req(wallet_handle, did_trustee, payment_method, '[txnType1:1, txnType2:2]')
