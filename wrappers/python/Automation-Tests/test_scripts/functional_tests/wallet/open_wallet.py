"""
Created on Dec 08, 2017

@author: khoi.ngo
Implementing test case open_wallet with valid value.
"""
from indy.error import IndyError
from libraries.test_scenario_base import TestScenarioBase
from libraries.utils import perform
from libraries import common
from libraries.result import Status


class OpenWallet(TestScenarioBase):
    async def execute_test_steps(self):
        print("OpenWallet test started")
        # 1. Create and open a pool
        self.steps.add_step("Create and open a pool")
        self.pool_handle = await perform(self.steps,
                                         common.create_and_open_pool,
                                         self.pool_name,
                                         self.pool_genesis_txn_file)

        # 2. Create and open a wallet
        self.steps.add_step("Create and open a wallet")
        returned_code = await perform(self.steps,
                                      common.create_and_open_wallet,
                                      self.pool_name, self.wallet_name)

        # 3. Verify that user is able to open a new wallet
        self.steps.add_step("Verify the response code of open_wallet API.")
        if not isinstance(returned_code, IndyError):
            self.wallet_handle = returned_code  # using for post-condition
            self.steps.get_last_step().set_status(Status.PASSED)
        else:
            self.steps.get_last_step().set_message(
                "Failed. Cannot open the wallet which was created.")

        print("OpenWallet test completed")


if __name__ == '__main__':
    OpenWallet().execute_scenario()
