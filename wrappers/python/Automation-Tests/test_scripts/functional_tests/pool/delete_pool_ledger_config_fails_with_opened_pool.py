"""
Created on Dec 12, 2017

@author: nhan.nguyen
"""

from indy import pool
from indy.error import ErrorCode

from libraries import utils
from libraries import common, constant
from test_scripts.functional_tests.pool.pool_test_base import PoolTestBase


class TestOpenPoolLedgerConfig(PoolTestBase):
    async def execute_test_steps(self):
        # 1. Create pool ledger config.
        self.steps.add_step("Create pool ledger config")
        await utils.perform(self.steps, common.create_pool_ledger_config,
                            self.pool_name, constant.pool_genesis_txn_file)
        # 2. Open pool ledger.
        self.steps.add_step("Open pool ledger")
        self.pool_handle = await \
            utils.perform(self.steps, pool.open_pool_ledger,
                          self.pool_name, None)

        # 3. Delete opened pool ledger
        # and verify that a opened pool ledger cannot be deleted.
        self.steps.add_step("Delete opened pool ledger and verify that"
                            " opened pool ledger cannot be deleted")
        error_code = ErrorCode.CommonInvalidState
        await utils.perform_with_expected_code(self.steps,
                                               pool.delete_pool_ledger_config,
                                               self.pool_name,
                                               expected_code=error_code)
        # Check pool config folder still exists.
        error_message = "Pool config folder does not exists"
        utils.check(self.steps, error_message,
                    condition=lambda: utils.check_pool_exist(self.pool_name))


if __name__ == "__main__":
    TestOpenPoolLedgerConfig().execute_scenario()
