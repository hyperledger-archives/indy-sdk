from indy import payment

import json
import pytest


TEXT = 'some agreement text'
VERSION = '1.0.0'
ACCEPTANCE_MECH_TYPE = 'acceptance type 1'
TAA_DIGEST = '050e52a57837fff904d3d059c8a123e3a04177042bf467db2b2c27abd8045d5e'
TIME_OF_ACCEPTANCE = 123379200
EXTRA = {
    "data": "some extra",
}
TAA_ACCEPTANCE = {
    "mechanism": ACCEPTANCE_MECH_TYPE,
    "taaDigest": TAA_DIGEST,
    "time": TIME_OF_ACCEPTANCE
}


@pytest.mark.asyncio
async def test_prepare_payment_extra_with_acceptance_data():
    extra = await payment.prepare_payment_extra_with_acceptance_data(json.dumps(EXTRA), TEXT, VERSION, None,
                                                                     ACCEPTANCE_MECH_TYPE, TIME_OF_ACCEPTANCE)
    expected_extra = EXTRA
    EXTRA["taaAcceptance"] = TAA_ACCEPTANCE

    extra = json.loads(extra)
    assert extra == expected_extra


@pytest.mark.asyncio
async def test_prepare_payment_extra_with_acceptance_data_for_empty_extra():
    extra = await payment.prepare_payment_extra_with_acceptance_data(None, TEXT, VERSION, None,
                                                                     ACCEPTANCE_MECH_TYPE, TIME_OF_ACCEPTANCE)
    expected_extra = {
        "taaAcceptance": TAA_ACCEPTANCE
    }

    extra = json.loads(extra)
    assert extra == expected_extra
