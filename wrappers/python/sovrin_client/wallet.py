from typing import Callable
from . import SovrinError

class SovrinWalletInterface(object):

    """TODO: document it"""

    def __init__(self):
        raise NotImplementedError("Should have implemented this")

    def create(self, name, config, credentials):
        raise NotImplementedError("Should have implemented this")

    def open(self, name, config, credentials):
        raise NotImplementedError("Should have implemented this")

    def set(self, key, subkey, value):
        raise NotImplementedError("Should have implemented this")

    def get(self, key, subkey, value, value_lifetime):
        raise NotImplementedError("Should have implemented this")

    def close(self):
        raise NotImplementedError("Should have implemented this")

    def delete(self, name: str):
        raise NotImplementedError("Should have implemented this")


class SovrinWallet(object):

    """TODO: document it"""

    def __init__(self) -> None:
        return None

    def register_wallet_type(self,
                             wtype: str,
                             impl: SovrinWalletInterface) -> SovrinError:
        return SovrinError.Success

    def create_wallet(self,
                      command_handle: int,
                      pool_name: str,
                      name: str,
                      wtype: str,
                      config: str,
                      credentials: str,
                      cb: Callable[[int, SovrinError], None]) -> SovrinError:
        cb(0, SovrinError.Success)
        return SovrinError.Success

    def open_wallet(self,
                    command_handle: int,
                    pool_handle: int,
                    name: str,
                    config: str,
                    cb: Callable[[int, SovrinError, int], None]) -> SovrinError:
        return SovrinError.Success

    def close_wallet(self,
                     command_handle: int,
                     cb: Callable[[int, SovrinError], None]) -> SovrinError:
        return SovrinError.Success

    def delete_wallet(self,
                      command_handle: int,
                      name:str,
                      cb: Callable[[int, SovrinError], None]) -> SovrinError:
        return SovrinError.Success

    def set_seq_no_for_value(self,
                             command_handle: int,
                             wallet_key: str,
                             cb: Callable[[int, SovrinError], None]) -> SovrinError:
        return SovrinError.Success
