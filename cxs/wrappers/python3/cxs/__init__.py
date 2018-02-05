from ctypes import CDLL
import os

def find_shared_object():
    file_dir = os.path.dirname(__file__)
    path = os.path.join(file_dir, "lib", "libcxs.so")
    return path

cxs = CDLL(find_shared_object())

if __name__ == "__main__":
    pass
