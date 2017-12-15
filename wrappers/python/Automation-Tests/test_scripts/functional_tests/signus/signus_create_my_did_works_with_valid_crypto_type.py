"""
Created on Dec 12, 2017

@author: nhan.nguyen
"""

import json

from indy import signus

from libraries import utils
from libraries import common, constant
from test_scripts.functional_tests.signus.signus_test_base\
    import SignusTestBase


class TestCreateDidWithValidCryptoType(SignusTestBase):
    async def execute_test_steps(self):
        # 1. Create wallet.
        # 2. Open wallet.
        self.wallet_handle = await \
            common.create_and_open_wallet_for_steps(self.steps,
                                                    self.wallet_name,
                                                    self.pool_name)

        # 3. Create did and verkey with valid crypto type.
        self.steps.add_step("Create did and verkey with valid crypto type")
        did_json = json.dumps({"seed": constant.seed_my1,
                               "crypto_type": "ed25519"})
        (my_did, my_verkey) = await \
            utils.perform(self.steps, signus.create_and_store_my_did,
                          self.wallet_handle, did_json)

        # 4. Check created did.
        self.steps.add_step("Check created did")
        utils.check(self.steps, error_message="Created did is invalid",
                    condition=lambda: my_did == constant.did_my1)

        # 5. Check created verkey.
        self.steps.add_step("Check created verkey")
        utils.check(self.steps, error_message="Created verkey is invalid",
                    condition=lambda: my_verkey == constant.verkey_my1)


if __name__ == "__main__":
    TestCreateDidWithValidCryptoType().execute_scenario()
