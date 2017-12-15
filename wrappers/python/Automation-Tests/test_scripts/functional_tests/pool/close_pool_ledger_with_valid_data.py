"""
Created on Dec 8, 2017

@author: nhan.nguyen
"""

from indy import pool
from libraries import utils
from libraries import common, constant
from test_scripts.functional_tests.pool.pool_test_base import PoolTestBase


class TestClosePoolLedgerConfig(PoolTestBase):
    async def execute_test_steps(self):
        # 1. Create pool ledger config.
        # 2. Open pool ledger.
        self.pool_handle = await \
            common.create_and_open_pool_ledger_for_steps(self.steps,
                                                         self.pool_name,
                                                         constant.
                                                         pool_genesis_txn_file)

        # 3. Close pool ledger.
        self.steps.add_step("Close pool ledger")
        result = await utils.perform(self.steps, pool.close_pool_ledger,
                                     self.pool_handle, ignore_exception=True)

        # 4. Verify that pool ledger is closed successfully.
        self.steps.add_step("Verify that pool ledger is closed successfully")
        error_message = "Cannot close opened pool ledger"
        if utils.check(self.steps, error_message,
                       condition=lambda: result is None):
            # prevent post-condition close pool ledger again.
            self.pool_handle = None


if __name__ == "__main__":
    TestClosePoolLedgerConfig().execute_scenario()
