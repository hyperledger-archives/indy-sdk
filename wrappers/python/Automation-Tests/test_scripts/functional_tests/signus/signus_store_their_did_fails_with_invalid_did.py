"""
Created on Dec 12, 2017

@author: nhan.nguyen
"""

import json

from indy import signus
from indy.error import ErrorCode
from utilities import utils, common
from test_scripts.functional_tests.signus.signus_test_base \
    import SignusTestBase


class TestStoreInvalidDidIntoWallet(SignusTestBase):
    async def execute_test_steps(self):
        # 1. Create wallet.
        # 2. Open wallet.
        self.wallet_handle = await \
            common.create_and_open_wallet_for_steps(self.steps,
                                                    self.wallet_name,
                                                    self.pool_name)

        # 4. Store an invalid did into wallet and
        # verify that invalid did cannot be stored.
        self.steps.add_step("Store an invalid did into wallet and"
                            " verify that invalid did cannot be stored")

        invalid_did_json = json.dumps({"did": "invalidDID"})
        error_code = ErrorCode.CommonInvalidStructure
        await utils.perform_with_expected_code(self.steps,
                                               signus.store_their_did,
                                               self.wallet_handle,
                                               invalid_did_json,
                                               expected_code=error_code)


if __name__ == "__main__":
    TestStoreInvalidDidIntoWallet().execute_scenario()
