"""
Created on Nov 9, 2017

@author: nhan.nguyen

Containing classes to make the test result as a json.
"""

import json
import time
import os
import errno
from enum import Enum
from utilities import utils


class Status(str, Enum):
    PASSED = "Passed"
    FAILED = "Failed"


class TestResult:
    __TEST_CASE = "testcase"
    __RESULT = "result"
    __START_TIME = "starttime"
    __DURATION = "duration"
    __RUN = "run"
    __STEP = "step"
    __STATUS = "status"
    __MESSAGE = "message"

    __json_dir = (os.path.join(os.path.dirname(__file__), "..",) +
                  "/test_output/test_results/")

    def __init__(self, test_case_name):
        """
        Constructor of a TestResult instance.

        :param test_case_name: (optional) name of test case.
        """
        TestResult.__init_output_folder()
        self.__test_result = {}  # Store information of a test case
        self.__run = []  # Store information of steps in test case
        self.__test_result[TestResult.__TEST_CASE] = test_case_name
        self.__test_result[TestResult.__RESULT] = Status.PASSED
        self.__test_result[TestResult.__START_TIME] = \
            str(time.strftime("%Y-%m-%d_%H-%M-%S"))
        self.__json_file_path = \
            "{}{}_{}.json".format(TestResult.__json_dir,
                                  self.__test_result[TestResult.__TEST_CASE],
                                  self.__test_result[TestResult.__START_TIME])

    def set_result(self, result):
        """
        Set a result (PASSED or FAILED) for test case.

        :param result: (optional) result of test.
        """
        self.__test_result[TestResult.__RESULT] = result

    def set_duration(self, duration):
        """
        Set duration for test.

        :param duration: (second).
        """
        self.__test_result[TestResult.__DURATION] = round(duration * 1000)

    def set_step_status(self, step_summary: str, status: str = Status.PASSED,
                        message: str = None):
        """
        Set status and message for specify step.

        :param step_summary: (optional) title of step.
        :param status: (optional) PASSED or FAILED.
        :param message: anything that involve to step like Exception, Log,...
        """
        temp = {TestResult.__STEP: step_summary, TestResult.__STATUS: status,
                TestResult.__MESSAGE: message}
        self.__run.append(temp)

    def add_step(self, step):
        """
        Add a step to report.

        :param step: (optional) a Step object in step.py
        """
        if not step:
            return
        temp = {TestResult.__STEP: step.get_name(),
                TestResult.__STATUS: step.get_status(),
                TestResult.__MESSAGE: step.get_message()}
        self.__run.append(temp)

    def write_result_to_file(self):
        """
        Write the result as json.
        """
        self.__test_result[TestResult.__RUN] = self.__run
        with open(self.__json_file_path, "w+") as outfile:
            json.dump(self.__test_result, outfile,
                      ensure_ascii=False, indent=2)
            utils.print_ok_blue(
                "\nJson file has been written at: {}\n".
                format(self.__json_file_path))

    def set_test_failed(self):
        """
        Set status of test to FAILED.
        """
        self.set_result(Status.FAILED)

    def set_test_passed(self):
        """
        Set status of test to PASSED.
        """
        self.set_result(Status.PASSED)

    def get_test_status(self) -> str:
        """
        Get the status of test.

        :return: test status.
        """
        return self.__test_result[TestResult.__RESULT]

    @staticmethod
    def __init_output_folder():
        """
        Create test_output directory if it not exist.

        :raise OSError.
        """
        try:
            os.makedirs(TestResult.__json_dir)
        except OSError as e:
            if e.errno != errno.EEXIST:
                raise e
