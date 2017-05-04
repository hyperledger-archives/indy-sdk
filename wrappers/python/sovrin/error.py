from enum import Enum

class SovrinErrorCode(Enum):
    Success = 0
    CommonInvalidParam1 = 100


class SovrinError(Exception):
    error_code: SovrinErrorCode

    def __init__(self, error_code: SovrinErrorCode):
        self.error_code = error_code
