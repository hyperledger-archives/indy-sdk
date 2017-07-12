import ctypes
import logging
import sys


def load_cdll():
    libindy_prefix_mapping = {'darwin': 'lib', 'linux': 'lib', 'linux2': 'lib', 'win32': ''}
    libindy_suffix_mapping = {'darwin': '.dylib', 'linux': '.so', 'linux2': '.so', 'win32': '.dll'}

    os_name = sys.platform
    logging.debug("Detected OS name is: %s", os_name)

    try:
        libindy_prefix = libindy_prefix_mapping[os_name]
        libindy_suffix = libindy_suffix_mapping[os_name]
    except KeyError:
        logging.error("OS isn't supported: %s", os_name)
        raise OSError("OS isn't supported: %s", os_name)

    libindy_name = libindy_prefix + 'indy' + libindy_suffix
    logging.debug("Resolved libindy name is: %s", libindy_name)

    try:
        return ctypes.CDLL(libindy_name)
    except OSError as e:
        logging.error("Can't load libindy: %s", e)
        raise e


# load indy sdk C library
cdll = load_cdll()
