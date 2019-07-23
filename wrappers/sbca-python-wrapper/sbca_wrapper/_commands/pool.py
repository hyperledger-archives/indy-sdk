from typing import Optional, Union

from .._command import LibindyCommand


class Pool:

    @staticmethod
    @LibindyCommand('indy_create_pool_ledger_config')
    async def create_pool_config(
            config_name: str,
            config: Optional[Union[dict, str]]
    ):
        """"""
        pass

    @staticmethod
    @LibindyCommand('indy_delete_pool_ledger_config')
    async def delete_pool_config(
            config_name: str
    ):
        """"""
        pass

    @staticmethod
    @LibindyCommand('indy_open_pool_ledger')
    async def open_pool_connection(
            config_name: str,
            config: Optional[Union[dict, str]]
    ) -> int:
        """"""
        pass

    @staticmethod
    @LibindyCommand('indy_close_pool_ledger')
    async def close_pool_connection(
            pool_handle: int
    ):
        """"""
        pass

    @staticmethod
    @LibindyCommand('indy_refresh_pool_ledger')
    async def refresh_local_pool_ledger(
            pool_handle: int
    ):
        """"""
        pass

    @staticmethod
    @LibindyCommand('indy_list_pools')
    async def list_local_pool_ledgers() -> list:
        """"""
        pass

    @staticmethod
    @LibindyCommand('indy_set_protocol_version',
                    protocol_version=lambda arg: arg)
    async def set_protocol_version(
            protocol_version: int
    ):
        """"""
        pass
