"""
Created on Dec 08, 2017

@author: khoi.ngo
Implementing test case create_wallet with valid value.
"""
import os
from indy import wallet
from indy.error import IndyError
from utilities.constant import Color
from utilities import common, constant
from utilities.utils import perform
from utilities.result import Status
from utilities.test_scenario_base import TestScenarioBase


class CreateWallet(TestScenarioBase):
    async def execute_postcondition_steps(self):
        common.clean_up_pool_and_wallet_folder(self.pool_name,
                                               self.wallet_name)

    async def execute_test_steps(self):
        print("CreateWallet test started")
        try:
            # 1. Create and open a pool
            self.steps.add_step("Create pool Ledger")
            self.pool_handle = await perform(self.steps,
                                             common.create_and_open_pool,
                                             self.pool_name,
                                             self.pool_genesis_txn_file)

            # 2. Create a wallet
            self.steps.add_step("Create wallet")
            returned_code = await perform(self.steps, wallet.create_wallet,
                                          self.pool_name, self.wallet_name,
                                          None, None, None)

            # 3. Verify new created wallet folder exists
            self.steps.add_step("Verify new created wallet folder exists")
            wallet_path = constant.work_dir + "/wallet/" + self.wallet_name
            result = os.path.exists(wallet_path)
            if (not isinstance(returned_code, IndyError)) and (result is True):
                self.steps.get_last_step().set_status(Status.PASSED)
            else:
                self.steps.get_last_step().set_message(
                    "Failed. Cannot find the wallet which was created.")
        except Exception as e:
            print(Color.FAIL + "\n\t{}\n".format(str(e)) + Color.ENDC)
        print("CreateWallet test completed")


if __name__ == '__main__':
    CreateWallet().execute_scenario()
