from typing import Callable
from . import SovrinError

class Wallet(object):

    """TODO: document it"""

    async def create_wallet(pool_name: str,
                            name: str,
                            xtype: str,
                            config: str,
                            credentials: str) -> None:
        pass

    async def open_wallet(pool_handle: int,
                          name: str,
                          config: str) -> int:
        return -1

    async def close_wallet(wallet_handle: int) -> None:
        pass

    async def delete_wallet(name:str) -> None:
        pass

    async def set_seq_no_for_value(wallet_key: str, seq_num: str) -> None:
        pass
