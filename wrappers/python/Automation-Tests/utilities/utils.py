"""
Created on Nov 9, 2017

@author: khoi.ngo

Containing all functions used by several test steps on test scenarios.
"""
import os
from indy.error import IndyError
from utilities import constant
from utilities.result import Status
from utilities.step import Steps


def generate_random_string(prefix="", suffix="", size=20):
    """
    Generate random string .

    :param prefix:  (optional) Prefix of a string.
    :param suffix:  (optional) Suffix of a string.
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


def exit_if_exception(result):
    """
    If "result" is an exception then raise the "result".
    Unless "result" is an exception then return the "result".
    :param result: the value that you want to check.
    :return: "result" if it is not an exception.
    """
    if isinstance(result, IndyError) or (isinstance(result, Exception)):
        exit(1)
    else:
        return result


def compare_json(js1, js2):
    return js1.items() <= js2.items()


async def perform(steps, func, *args, ignore_exception=False):
    """
    Execute an function and set status, message for the last test step depend
    on the result of the function.

    :param steps: list of test steps.
    :param func: executed function.
    :param args: argument of function.
    :param ignore_exception: (optional) raise exception or not.
    :return: the result of function of the exception that the function raise.
    """
    try:
        result = await func(*args)
        steps.get_last_step().set_status(Status.PASSED)
    except IndyError as E:
        print_error(constant.INDY_ERROR.format(str(E)))
#         steps.get_last_step().set_message(str(E))
        steps.get_last_step().set_status(Status.FAILED, str(E))
        result = E
    except Exception as Ex:
        print_error(constant.EXCEPTION.format(str(Ex)))
#         steps.get_last_step().set_message(str(Ex))
        steps.get_last_step().set_status(Status.FAILED, str(Ex))
        result = Ex

    if isinstance(result, Exception) and not ignore_exception:
        raise result

    return result


async def perform_with_expected_code(steps, func, *args, expected_code=0):
    """
    Execute the "func" with expectation that the "func" raise an IndyError that
    IndyError.error_code = "expected_code".

    :param steps: list of test steps.
    :param func: executed function.
    :param args: arguments of "func".
    :param expected_code: (optional) the error code that you expect
                          in IndyError.
    :return: exception if the "func" raise it without "expected_code".
             'None' if the "func" run without any exception of the exception
             contain "expected_code".
    """
    try:
        await func(*args)
        message = "Expected exception %s but not." % str(expected_code)
        steps.get_last_step().set_status(Status.FAILED, message)
        return None
    except IndyError as E:
        if E.error_code == expected_code:
            steps.get_last_step().set_status(Status.PASSED)
            return True
        else:
            print_error(constant.INDY_ERROR.format(str(E)))
            steps.get_last_step().set_status(Status.FAILED, str(E))
            return E
    except Exception as Ex:
        print_error(constant.EXCEPTION.format(str(Ex)))
        steps.get_last_step().set_status(Status.FAILED, str(Ex))
        return Ex


def run_async_method(method, time_out=None):
    """
    Run async method until it complete or until the time is over.

    :param method: The method want to run with event loop.
    :param time_out:

    @note: We can customize this method to adapt different situations
           in the future.
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

    :param test_result: the object result was collected into test case.
    :param steps: list of steps.
    :param begin_time: time that the test begin.
    :param logger: The object captures screen log.
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

    :param steps: list step of test case.
    :param expected_response: expected json.
    :param response: actual json.
    """
    try:
        assert expected_response.items() <= response.items()
        steps.get_last_step().set_status(Status.PASSED)
    except AssertionError as e:
        message = constant.JSON_INCORRECT.format(str(e))
        steps.get_last_step().set_status(Status.FAILED, message)


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

    :param message:
    :param color:
    """
    print(color + message + constant.Color.ENDC)


def print_error(message: str):
    """
    Print message onto console with "Fail" color.

    :param message:
    """
    print_with_color(message, constant.Color.FAIL)


def print_header(message: str):
    """
    Print message onto console with "Header" color.

    :param message:
    """
    print_with_color(message, constant.Color.HEADER)


def print_ok_green(message: str):
    """
    Print message onto console with "OK_GREEN" color.

    :param message:
    """
    print_with_color(message, constant.Color.OKGREEN)


def print_ok_blue(message: str):
    """
    Print message onto console with "OK_BLUE" color.

    :param message:
    """
    print_with_color(message, constant.Color.OKBLUE)


def print_test_result(test_name, test_status):
    if test_status == Status.PASSED:
        print_with_color("Test case: {} ----> {}\n".
                         format(test_name, "PASSED"),
                         constant.Color.OKGREEN)
    else:
        print_error("Test case: {} ----> {}\n".format(test_name, "FAILED"))


def check(steps: Steps, error_message: str, condition) -> bool:
    """
    Check if the condition are return True.
    Set message into last step if the condition return False.

    :param steps: list step of test case.
    :param error_message: message to set if condition return False.
    :param condition: a callable.
    :return: True or False depend on result of "condition".
    """
    if steps:
        step = steps.get_last_step()
        if not callable(condition):
            raise ValueError("The 'condition' argument "
                             "must be a callable object")
        else:
            if not condition():
                raise ValueError(error_message)
            else:
                step.set_status(Status.PASSED)
                return True

    return False


def create_claim_offer(issuer_did: str, schema_seq: int):
    """
    Return a claim offer.
    :param issuer_did: create by signus.create_and_store_did.
    :param schema_seq:
    :return: claim offer.
    """
    return {"issuer_did": issuer_did, "schema_seq_no": schema_seq}
