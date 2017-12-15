"""
Created on Nov 9, 2017

@author: khoi.ngo

Containing all functions used by several test steps on test scenarios.
"""
import os
from indy.error import IndyError
from libraries import constant
from libraries.result import Status
from libraries.step import Steps


def generate_random_string(prefix="", suffix="", size=20):
    """
    Generate random string .

    :param prefix: (optional) Prefix of a string.
    :param suffix: (optional) Suffix of a string.
    :param size: (optional) Max length of a string (include prefix and suffix)
    :return: The random string.
    """
    import random
    import string
    left_size = size - len(prefix) - len(suffix)
    random_str = ""
    if left_size > 0:
        random_str = ''.join(
            random.choice(string.ascii_uppercase +
                          string.digits) for _ in range(left_size))
    else:
        print("Warning: Length of prefix and suffix more than %s chars"
              % str(size))
    result = str(prefix) + random_str + str(suffix)
    return result


def exit_if_exception(code):
    """
    If "code" is an exception then raise the "code".
    Unless "code" is an exception then return the "code".
    :param code: (optional) code that you want to check.
    :return: "code" if it is not an exception.
    """
    if isinstance(code, IndyError) or (isinstance(code, Exception)):
        exit(1)
    else:
        return code


def compare_json(js1, js2):
    return js1.items() <= js2.items()


async def perform(steps, func, *args, ignore_exception=False):
    """
    Execute an function and set status, message for the last test step depend
    on the result of the function.

    :param steps: (optional) list of test steps.
    :param func: (optional) executed function.
    :param args: argument of function.
    :param ignore_exception: raise exception or not.
    :return: the result of function of the exception that the function raise.
    """
    try:
        result = await func(*args)
        steps.get_last_step().set_status(Status.PASSED)
    except IndyError as E:
        print_error(constant.INDY_ERROR.format(str(E)))
        steps.get_last_step().set_message(str(E))
        steps.get_last_step().set_status(Status.FAILED)
        result = E
    except Exception as Ex:
        print_error(constant.EXCEPTION.format(str(Ex)))
        steps.get_last_step().set_message(str(Ex))
        steps.get_last_step().set_status(Status.FAILED)
        result = Ex

    if not ignore_exception:
        exit_if_exception(result)

    return result


async def perform_with_expected_code(steps, func, *agrs, expected_code=0):
    """
    Execute the "func" with expectation that the "func" raise an IndyError that
    IndyError.error_code = "expected_code".

    :param steps: (optional) list of test steps.
    :param func: (optional) executed function.
    :param agrs: arguments of "func".
    :param expected_code: the error code that you expect in IndyError.
    :return: exception if the "func" raise it without "expected_code".
             'None' if the "func" run without any exception of the exception
             contain "expected_code".
    """
    try:
        await func(*agrs)
        steps.get_last_step().set_message("Can execute without exception.")
        steps.get_last_step().set_status(Status.FAILED)
        return None
    except IndyError as E:
        if E.error_code == expected_code:
            steps.get_last_step().set_status(Status.PASSED)
            return True
        else:
            print_error(constant.INDY_ERROR.format(str(E)))
            steps.get_last_step().set_message(str(E))
            return E
    except Exception as Ex:
        print_error(constant.EXCEPTION.format(str(Ex)))
        return Ex


def run_async_method(method, time_out=None):
    """
    Run async method until it complete or until the time is over.

    :param method: (optional).
    :param time_out:
    """
    import asyncio
    loop = asyncio.get_event_loop()
    if not time_out:
        loop.run_until_complete(method())
    else:
        loop.run_until_complete(asyncio.wait_for(method(), time_out))


def make_final_result(test_result, steps, begin_time, logger):
    """
    Making a test result.

    :param test_result: (optional).
    :param steps: (optional) list of steps.
    :param begin_time: (optional) time that the test begin.
    :param logger: (optional).
    """
    import time
    for step in steps:
        test_result.add_step(step)
        if step.get_status() == Status.FAILED:
            print('%s: ' % str(step.get_id()) + constant.Color.FAIL +
                  'failed\nMessage: ' + step.get_message() +
                  constant.Color.ENDC)
            test_result.set_test_failed()

    test_result.set_duration(time.time() - begin_time)
    test_result.write_result_to_file()
    logger.save_log(test_result.get_test_status())


def verify_json(steps, expected_response, response):
    """
    Verify two json are equal.
    :param steps: (optional) list step of test case.
    :param expected_response: (optional) expected json.
    :param response: (optional) actual json.
    """
    try:
        assert expected_response.items() <= response.items()
        steps.get_last_step().set_status(Status.PASSED)
    except AssertionError as e:
        steps.get_last_step().set_message(
            constant.JSON_INCORRECT.format(str(e)))


def check_pool_exist(pool_name: str) -> bool:
    """
    Check whether pool config exist or not.

    :param pool_name:
    :return: bool
    """
    if not pool_name:
        return False
    return os.path.exists(constant.work_dir + "/pool/" + pool_name)


def print_with_color(message: str, color: str):
    """
    Print a message with specified color onto console.

    :param message: (optional)
    :param color: (optional)
    """
    print(color + message + constant.Color.ENDC)


def print_error(message: str):
    """
    Print message onto console with "Fail" color.

    :param message: (optional)
    """
    print_with_color(message, constant.Color.FAIL)


def print_header(message: str):
    """
    Print message onto console with "Header" color.

    :param message: (optional)
    """
    print_with_color(message, constant.Color.HEADER)


def print_ok_green(message: str):
    """
    Print message onto console with "OK_GREEN" color.

    :param message: (optional)
    """
    print_with_color(message, constant.Color.OKGREEN)


def print_ok_blue(message: str):
    """
    Print message onto console with "OK_BLUE" color.

    :param message: (optional)
    """
    print_with_color(message, constant.Color.OKBLUE)


def check(steps: Steps, error_message: str, condition) -> bool:
    """
    Check if the condition are return True.
    Set message into last step if the condition return False.

    :param steps: (optional) list step of test case.
    :param error_message: message to set if condition return False.
    :param condition: (optional) a callable.
    :return: True or False depend on result of "condition".
    """
    if steps:
        step = steps.get_last_step()
        if not callable(condition):
            step.set_status(Status.FAILED)
            step.set_message("The 'condition' argument "
                             "must be a callable object")
        else:
            if not condition():
                step.set_status(Status.FAILED)
                if error_message:
                    temp_message = (step.get_message + "\n") \
                        if step.get_message() else ""
                    step.set_message(temp_message + error_message)
            else:
                step.set_status(Status.PASSED)
                return True

    return False
