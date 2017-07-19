from tests.utils import storage
from indy import ledger
from indy.error import ErrorCode, IndyError

import pytest
import logging

logging.basicConfig(level=logging.DEBUG)


@pytest.fixture(autouse=True)
def before_after_each():
    storage.cleanup()
    yield
    storage.cleanup()


@pytest.mark.asyncio
async def test_build_nym_request_works_for_invalid_identifier():
    identifier = "invalid_base58_identifier"
    dest = "FYmoFw55GeQH7SRFa37dkx1d2dZ3zUF8ckg7wmL7ofN4"

    try:
        await ledger.build_nym_request(identifier, dest, None, None, None)
        raise Exception("Failed")
    except Exception as e:
        assert type(IndyError(ErrorCode.CommonInvalidStructure)) == type(e) and \
               IndyError(ErrorCode.CommonInvalidStructure).args == e.args
