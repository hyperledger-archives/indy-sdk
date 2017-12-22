"""
Created on Dec 14, 2017

@author: nhan.nguyen
"""

import json

from indy import signus
from indy.error import ErrorCode
from utilities import utils, common, constant
from test_scripts.functional_tests.signus.signus_test_base \
    import SignusTestBase


class TestSignWithValidData(SignusTestBase):
    async def execute_test_steps(self):
        # 1. Create wallet.
        # 2. Open wallet.
        self.wallet_handle = await \
            common.create_and_open_wallet_for_steps(self.steps,
                                                    self.wallet_name,
                                                    self.pool_name)

        # 3. Create did and verkey with valid seed.
        self.steps.add_step("Create did and verkey with valid seed")
        did_json = json.dumps({"seed": constant.seed_my1})
        (my_did, _) = await \
            utils.perform(self.steps, signus.create_and_store_my_did,
                          self.wallet_handle, did_json)

        # 4. Use "signus.sign" to sign with invalid wallet handle
        # and verify thay user cannot sign successfully.
        self.steps.add_step("Use 'signus.sign' to sign with invalid "
                            "wallet handle and verify thay user "
                            "cannot sign successfully")
        error_code = ErrorCode.WalletInvalidHandle
        await utils.perform_with_expected_code(self.steps, signus.sign,
                                               self.wallet_handle + 1, my_did,
                                               "Test signus".encode("UTF-8"),
                                               expected_code=error_code)


if __name__ == "__main__":
    TestSignWithValidData().execute_scenario()
