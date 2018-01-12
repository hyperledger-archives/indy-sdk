"""
Created on Dec 13, 2017

@author: nhan.nguyen
"""

import json

from indy import signus
from indy.error import ErrorCode
from utilities import common
from utilities import utils
from test_scripts.functional_tests.signus.signus_test_base \
    import SignusTestBase


class TestCreateDidWithInvalidSeed(SignusTestBase):
    async def execute_test_steps(self):
        # 1. Create wallet.
        # 2. Open wallet.
        self.wallet_handle = await \
            common.create_and_open_wallet_for_steps(self.steps,
                                                    self.wallet_name,
                                                    self.pool_name)

        # 3. Create did and verify that cannot create did with invalid seed.
        self.steps.add_step("Create did and verify that "
                            "cannot create did with invalid seed")
        invalid_seed_json = json.dumps({"seed": "invalidSeed"})
        error_code = ErrorCode.CommonInvalidStructure
        await utils.perform_with_expected_code(self.steps,
                                               signus.create_and_store_my_did,
                                               self.wallet_handle,
                                               invalid_seed_json,
                                               expected_code=error_code)


if __name__ == "__main__":
    TestCreateDidWithInvalidSeed().execute_scenario()
