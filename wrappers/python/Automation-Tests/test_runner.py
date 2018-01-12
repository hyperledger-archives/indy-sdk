"""
Created on Nov 24, 2017

@author: nhan.nguyen

Containing class for Test Runner.
"""

import subprocess
import os
import glob
import sys
import inspect
import importlib
import multiprocessing
import argparse
from utilities import utils, constant
from utilities.test_scenario_base import TestScenarioBase


class TestRunner:
    __default_dir = os.path.dirname(os.path.abspath(__file__))
    __test_script_dir = __default_dir + "/test_scripts"
    __reporter_dir = __default_dir + "/reporter.py"

    def __init__(self):
        self.__args = None
        self.__test_process = None
        self.__current_scenario = None
        self.__continue = True
        self.__catch_arg()
        pass

    def run(self):
        """
        Run all test scenario and then execute reporter if -html flag exist.
        """
        list_test_scenarios = self.__get_list_scenarios_in_folder()

        if not list_test_scenarios:
            utils.print_error(
                "\n{}\n".format(constant.ERR_CANNOT_FIND_ANY_TEST_SCENARIOS))
            exit(1)

        for test_scenario in list_test_scenarios:
            if self.__continue:
                self.__execute_test_scenario(test_scenario)

        self.__execute_reporter()
        utils.print_ok_green(
            "\n{}\n".format(constant.INFO_ALL_TEST_HAVE_BEEN_EXECUTED))

    def __catch_arg(self):
        """
        Catch args for TestRunner in sys.argv.
        """
        arg_parser = argparse.ArgumentParser()
        arg_parser.add_argument("-d", "--directory", dest="directory",
                                default="", nargs="?",
                                help="directory of test "
                                     "scenarios (not recursive)")
        arg_parser.add_argument("-rd", "--recur_directory",
                                dest="recur_directory", default="", nargs="?",
                                help="directory of test scenarios (recursive)")
        arg_parser.add_argument("-t", "--timeout", dest="timeout", type=float,
                                help="timeout for each "
                                     "scenario (default: 300s)",
                                default=300, nargs="?")
        arg_parser.add_argument("-html", "--html_report", dest="report",
                                action="store_true", default=False,
                                help="if this flag is missing, html "
                                     "report would not be generated")
        arg_parser.add_argument("-l", "--keep_log", action="store_true",
                                help="keep all log file")
        self.__args = arg_parser.parse_args()
        if self.__args.timeout <= 0.0:
            print("Invalid timeout!")
            exit(1)

    def __execute_reporter(self):
        """
        Execute html_reporter if -html flag is exist in sys.argv.
        """
        if not self.__args.report:
            return
        cmd = "{} {}".format("python3.6", TestRunner.__reporter_dir)
        subprocess.call(cmd, shell=True)

    def __execute_test_scenario(self, test_scenario):
        """
        Execute test scenario.
        :param test_scenario: file that contain test scenarios.
        """
        if not test_scenario:
            return
        self.__current_scenario = test_scenario()
        process = multiprocessing.\
            Process(target=self.__current_scenario.execute_scenario,
                    kwargs={"time_out": self.__args.timeout})
        self.__test_process = process
        process.start()
        process.join()

    def __get_list_scenarios_in_folder(self):
        """
        Get all scenario in folder.
        Recursive to sub folder if "-rd" argument appear in sys.argv.
        :return: list test scenarios.
        """
        # If both directory and recur_directory are exist
        # then show "Invalid command" and exit.
        if self.__args.directory is not "" \
           and self.__args.recur_directory is not "":
            utils.print_error("\n{}\n".format(constant.ERR_COMMAND_ERROR))
            exit(1)
        recursive = False

        start_directory = ""
        if self.__args.directory is not "":
            start_directory = self.__args.directory
        elif self.__args.recur_directory is not "":
            start_directory = self.__args.recur_directory
            recursive = True

        if not start_directory:
            start_directory = TestRunner.__test_script_dir

        if not os.path.exists(start_directory):
            utils.print_error(
                "\n{}\n".format(constant.ERR_PATH_DOES_NOT_EXIST.
                                format(start_directory)))
            exit(1)

        list_files = []

        try:
            if recursive:
                for directory, _, _ in os.walk(start_directory):
                    list_files.extend(glob.glob(os.path.join(directory,
                                                             "*.py")))
            else:
                list_files.extend(glob.glob(os.path.join(start_directory,
                                                         "*.py")))
        except OSError:
            pass

        list_test_scenarios = []
        for file in list_files:
            sys.path.append(os.path.dirname(os.path.abspath(file)))
            test_module = \
                importlib.import_module(os.path.basename(file).replace(".py",
                                                                       ""))
            for name, cls in inspect.getmembers(test_module, inspect.isclass):
                if cls is not TestScenarioBase \
                        and issubclass(cls, TestScenarioBase):
                    list_test_scenarios.append(cls)

        return list_test_scenarios


if __name__ == "__main__":
    TestRunner().run()
