from typing import Callable
from . import IndyError

from . import libindy

class Wallet(object):

    """TODO: document it"""

    # extern indy_error_t indy_create_wallet(indy_handle_t  command_handle,
    #                                            const char*      pool_name,
    #                                            const char*      name,
    #                                            const char*      xtype,
    #                                            const char*      config,
    #                                            const char*      credentials,
    #                                            void            (*fn)(indy_handle_t xcommand_handle, indy_error_t err)
    #                                            );

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
