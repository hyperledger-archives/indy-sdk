"""
Created on Dec 14, 2017

@author: nhan.nguyen
"""

from indy import signus
from indy.error import ErrorCode
from utilities import utils, common, constant
from test_scripts.functional_tests.signus.signus_test_base \
    import SignusTestBase


class TestSignWithValidData(SignusTestBase):
    def __init__(self):
        super().__init__()

    async def execute_test_steps(self):
        # 1. Create wallet.
        # 2. Open wallet.
        self.wallet_handle = await \
            common.create_and_open_wallet_for_steps(self.steps,
                                                    self.wallet_name,
                                                    self.pool_name)

        # 4. Use "signus.sign" to sign with unknown did and
        # verify that user cannot sign successfully.
        self.steps.add_step("Use 'signus.sign' to sign with unknown did and "
                            "verify that user cannot sign successfully")
        error_code = ErrorCode.WalletNotFoundError
        await utils.perform_with_expected_code(self.steps, signus.sign,
                                               self.wallet_handle,
                                               constant.did_my1,
                                               "Test signus".encode("UTF-8"),
                                               expected_code=error_code)


if __name__ == "__main__":
    TestSignWithValidData().execute_scenario()
