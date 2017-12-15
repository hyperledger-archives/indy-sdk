"""
Created on Nov 22, 2017

@author: khoi.ngo

Containing the test base class.
"""

import inspect
import os
import time
import asyncio

from libraries import utils
from libraries import common, constant
from libraries.logger import Logger
from libraries.result import TestResult, Status
from libraries.step import Steps


class TestScenarioBase:
    """
    Test base....
    All test scenario should inherit from this class.
    This class controls the work flow and hold some general test data for test
    scenarios that inherit it.
    """
    def __init__(self):
        """
        Init test data.
        If the test case need some extra test date then
        just override this method.
        """
        self.test_name = os.path.splitext(
            os.path.basename(inspect.getfile(self.__class__)))[0]

        self.test_result = TestResult(self.test_name)
        self.steps = Steps()
        self.logger = Logger(self.test_name)
        self.pool_name = utils.generate_random_string("test_pool")
        self.wallet_name = utils.generate_random_string("test_wallet")
        self.pool_handle = None
        self.wallet_handle = None
        self.pool_genesis_txn_file = constant.pool_genesis_txn_file
        self.time_out = 300

    async def execute_precondition_steps(self):
        """
         Execute pre-condition of test scenario.
         If the test case need some extra step in pre-condition
         then just override this method.
        """
        common.clean_up_pool_and_wallet_folder(self.pool_name,
                                               self.wallet_name)

    async def execute_postcondition_steps(self):
        """
        Execute post-condition of test scenario.
        If the test case need some extra step in post-condition then
        just override this method.
        """
        await common.clean_up_pool_and_wallet(self.pool_name,
                                              self.pool_handle,
                                              self.wallet_name,
                                              self.wallet_handle)

    async def execute_test_steps(self):
        """
        The method where contain all main script of a test scenario.
        All test scenario inherit TestScenarioBase have
        to override this method.
        """
        pass

    def execute_scenario(self, time_out=None):
        """
        Execute the test scenario and control the
        work flow of this test scenario.
        """
        self.__init__()
        utils.print_with_color(
            "\nTest case: {} ----> started\n".format(self.test_name),
            constant.Color.BOLD)

        # Create new event loop for this test scenario.
        asyncio.set_event_loop(asyncio.new_event_loop())

        begin_time = time.time()
        if time_out:
            self.time_out = time_out

        try:
            utils.run_async_method(self.__execute_precondition_and_steps,
                                   self.time_out)
        except TimeoutError:
            utils.print_error("\n{}\n".format(constant.ERR_TIME_LIMITATION))
            self.steps.get_last_step().set_status(Status.FAILED)
            self.steps.get_last_step().set_message(
                constant.ERR_TIME_LIMITATION)
        except Exception as e:
            message = constant.EXCEPTION.format(str(e))
            utils.print_error("\n{}\n".format(message))
            self.steps.get_last_step().set_status(Status.FAILED)
            self.steps.get_last_step().set_message(message)
        finally:
            try:
                utils.run_async_method(self.execute_postcondition_steps)
            except Exception as e:
                utils.print_error("\n{}\n".format(str(type(e))))
                pass

            utils.make_final_result(self.test_result,
                                    self.steps.get_list_step(),
                                    begin_time, self.logger)
            asyncio.get_event_loop().close()

            utils.print_with_color(
                "Test case: {} ----> finished\n".format(self.test_name),
                constant.Color.BOLD)

    async def __execute_precondition_and_steps(self):
        """
        Execute precondition and test steps.
        """
        await self.execute_precondition_steps()
        await self.execute_test_steps()
