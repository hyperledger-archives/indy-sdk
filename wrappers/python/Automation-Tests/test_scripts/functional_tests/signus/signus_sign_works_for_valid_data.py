"""
Created on Dec 12, 2017

@author: nhan.nguyen
"""

import json

from indy import signus
from utilities import utils, common, constant
from test_scripts.functional_tests.signus.signus_test_base \
    import SignusTestBase


class TestSignWithValidData(SignusTestBase):
    def __init__(self):
        super().__init__()
        self.expected_signature = bytes([169, 215, 8, 225, 7, 107, 110, 9, 193,
                                         162, 202, 214, 162, 66, 238, 211, 63,
                                         209, 12, 196, 8, 211, 55, 27, 120, 94,
                                         204, 147, 53, 104, 103, 61, 60, 249,
                                         237, 127, 103, 46, 220, 223, 10, 95,
                                         75, 53, 245, 210, 241, 151, 191, 41,
                                         48, 30, 9, 16, 78, 252, 157, 206, 210,
                                         145, 125, 133, 109, 11])
        self.message = '{"reqId":1496822211362017764}'.encode('utf-8')

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

        # 4. Use created did to sign.
        self.steps.add_step("Use created did to sign")
        signature = await utils.perform(self.steps, signus.sign,
                                        self.wallet_handle, my_did,
                                        self.message)

        # 5. Check created signature.
        self.steps.add_step("Check created signature")
        utils.check(self.steps, error_message="Created signature is invalid",
                    condition=lambda: signature == self.expected_signature)


if __name__ == "__main__":
    TestSignWithValidData().execute_scenario()
