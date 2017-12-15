"""
Created on Dec 12, 2017

@author: nhan.nguyen
"""

import json
from indy import signus
from libraries import common
from libraries import utils
from test_scripts.functional_tests.signus.signus_test_base \
    import SignusTestBase


class TestStoreDidIntoWallet(SignusTestBase):
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

        # 4. Store created did into wallet.
        self.steps.add_step("Store created did into wallet")
        result = await utils.perform(self.steps, signus.store_their_did,
                                     self.wallet_handle,
                                     json.dumps({"did": their_did}))

        # 5. Verify that did is stored successfully.
        self.steps.add_step("Verify that did is stored successfully")
        utils.check(self.steps, error_message="Cannot store created did",
                    condition=lambda: result is None)


if __name__ == "__main__":
    TestStoreDidIntoWallet().execute_scenario()
