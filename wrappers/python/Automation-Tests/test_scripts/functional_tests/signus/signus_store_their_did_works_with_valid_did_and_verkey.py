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


class TestStoreDidAndVerkeyIntoWallet(SignusTestBase):
    async def execute_test_steps(self):
        # 1. Create wallet.
        # 2. Open wallet.
        self.wallet_handle = await \
            common.create_and_open_wallet_for_steps(self.steps,
                                                    self.wallet_name,
                                                    self.pool_name)

        # 3. Create did and verkey with empty json.
        self.steps.add_step("Create did and verkey with empty json")
        (their_did, their_verkey) = await \
            utils.perform(self.steps, signus.create_and_store_my_did,
                          self.wallet_handle, "{}")

        # 4. Store created did and verkey into wallet.
        self.steps.add_step("Store created did and verkey into wallet")
        did_key_json = json.dumps({"did": their_did, "verkey": their_verkey})
        result = await utils.perform(self.steps, signus.store_their_did,
                                     self.wallet_handle, did_key_json)

        # 5. Verify that did is stored successfully.
        self.steps.add_step("Verify that did is stored successfully")
        error_message = "Cannot store created did and verkey"
        utils.check(self.steps, error_message,
                    condition=lambda: result is None)


if __name__ == "__main__":
    TestStoreDidAndVerkeyIntoWallet().execute_scenario()
