"""
Created on Dec 12, 2017

@author: nhan.nguyen
"""

from indy import signus
from indy.error import ErrorCode
from utilities import utils, common
from test_scripts.functional_tests.signus.signus_test_base \
    import SignusTestBase


class TestStoreDidWithInvalidJson(SignusTestBase):
    async def execute_test_steps(self):
        # 1. Create wallet.
        # 2. Open wallet.
        self.wallet_handle = await \
            common.create_and_open_wallet_for_steps(self.steps,
                                                    self.wallet_name,
                                                    self.pool_name)

        # 3. Store did into wallet with invalid json and
        # verify that invalid json cannot be stored.
        self.steps.add_step("Store did into wallet with invalid json and "
                            "verify that invalid json cannot be stored")
        error_code = ErrorCode.CommonInvalidStructure
        invalid_json = '{"invalidField": "invalidValue"}'
        await  utils.perform_with_expected_code(self.steps,
                                                signus.store_their_did,
                                                self.wallet_handle,
                                                invalid_json,
                                                expected_code=error_code)


if __name__ == "__main__":
    TestStoreDidWithInvalidJson().execute_scenario()
