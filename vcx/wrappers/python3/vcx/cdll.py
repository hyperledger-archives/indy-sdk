from ctypes import CDLL
import os

LIBRARY = "libvcx.so"

def _cdll() -> CDLL:
    if not hasattr(_cdll, "cdll"):
        _cdll.cdll = _load_cdll()

    return _cdll.cdll


def _load_cdll() -> CDLL:
    file_dir = '/usr'
    path = os.path.join(file_dir, "lib", LIBRARY)
    try:
        res = CDLL(path)
        return res
    except OSError as e:
        raise e