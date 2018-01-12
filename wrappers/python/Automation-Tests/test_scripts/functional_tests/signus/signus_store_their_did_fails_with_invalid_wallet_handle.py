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


class TestStoreDidWithInvalidWalletHandle(SignusTestBase):
    async def execute_test_steps(self):
        # 1. Create wallet.
        # 2. Open wallet.
        self.wallet_handle = await \
            common.create_and_open_wallet_for_steps(self.steps,
                                                    self.wallet_name,
                                                    self.pool_name)

        # 3. Create did and verkey with empty json.
        self.steps.add_step("Create did and verkey with empty json")
        (their_did, _) = await \
            utils.perform(self.steps, signus.create_and_store_my_did,
                          self.wallet_handle, "{}")

        # 4. Store 'their_did' with invalid wallet handle and
        # verify that created did cannot be stored.
        self.steps.add_step("Store 'their_did' with invalid wallet handle and"
                            " verify that created did cannot be stored")
        did_json = json.dumps({"did": their_did})
        error_code = ErrorCode.WalletInvalidHandle
        await utils.perform_with_expected_code(self.steps,
                                               signus.store_their_did,
                                               self.wallet_handle + 1,
                                               did_json,
                                               expected_code=error_code)


if __name__ == "__main__":
    TestStoreDidWithInvalidWalletHandle().execute_scenario()
