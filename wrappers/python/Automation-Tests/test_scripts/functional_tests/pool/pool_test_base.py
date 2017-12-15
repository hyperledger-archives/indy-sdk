"""
Created on Dec 8, 2017

@author: nhan.nguyen

Containing a base class for pool testing.
"""

from libraries import common
from libraries.test_scenario_base import TestScenarioBase


class PoolTestBase(TestScenarioBase):
    def __init__(self):
        if self.__class__ is not PoolTestBase:
            super().__init__()

    async def execute_precondition_steps(self):
        common.delete_pool_folder(self.pool_name)

    async def execute_postcondition_steps(self):
        await common.close_and_delete_pool(self.pool_name, self.pool_handle)

    def execute_scenario(self, time_out=None):
        if self.__class__ is not PoolTestBase:
            super().execute_scenario(time_out)
