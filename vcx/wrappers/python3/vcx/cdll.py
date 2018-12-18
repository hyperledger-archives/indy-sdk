from ctypes import CDLL
import sys
from vcx.logging import set_logger

LIBRARY = "vcx"


def _cdll() -> CDLL:
    if not hasattr(_cdll, "cdll"):
        _cdll.cdll = _load_cdll()
        set_logger(_cdll.cdll)

    return _cdll.cdll


def _load_cdll() -> CDLL:
    prefix_mapping = {"darwin": "lib", "linux": "lib", "linux2": "lib", "win32": ""}
    suffix_mapping = {"darwin": ".dylib", "linux": ".so", "linux2": ".so", "win32": ".dll"}

    os_name = sys.platform

    try:
        prefix = prefix_mapping[os_name]
        suffix = suffix_mapping[os_name]
    except KeyError:
        raise OSError("OS isn't supported: %s", os_name)

    library_name = "{0}{1}{2}".format(prefix, LIBRARY, suffix)

    try:
        res = CDLL(library_name)
        return res
    except OSError as e:
        raise e
