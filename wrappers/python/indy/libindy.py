import ctypes
import sys
import logging


class LibIndy:
    _cdll = None

    @staticmethod
    def cdll() -> ctypes.CDLL:
        if LibIndy._cdll is None:
            LibIndy._cdll = LibIndy._load_cdll()
        return LibIndy._cdll

    @staticmethod
    def _load_cdll() -> ctypes.CDLL:
        logger = logging.getLogger(__name__)

        libindy_prefix_mapping = {'darwin': 'lib', 'linux': 'lib', 'linux2': 'lib', 'win32': ''}
        libindy_suffix_mapping = {'darwin': '.dylib', 'linux': '.so', 'linux2': '.so', 'win32': '.dll'}

        os_name = sys.platform
        logger.debug("Detected OS name is: %s", os_name)

        try:
            libindy_prefix = libindy_prefix_mapping[os_name]
            libindy_suffix = libindy_suffix_mapping[os_name]
        except KeyError:
            logger.error("OS isn't supported: %s", os_name)
            raise OSError("OS isn't supported: %s", os_name)

        libindy_name = libindy_prefix + 'indy' + libindy_suffix
        logger.debug("Resolved libindy name is: %s", libindy_name)

        try:
            return ctypes.CDLL(libindy_name)
        except OSError as e:
            logger.error("Can't load libindy: %s", e)
            raise e
