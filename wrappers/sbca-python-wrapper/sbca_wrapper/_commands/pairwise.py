from typing import Optional

from .._command import LibindyCommand


class Pairwise:

    @staticmethod
    @LibindyCommand('indy_is_pairwise_exists')
    async def pairwise_exists(
            wallet_handle: int,
            foreign_did: str
    ) -> bool:
        """"""
        pass

    @staticmethod
    @LibindyCommand('indy_create_pairwise')
    async def create_pairwise(
            wallet_handle: int,
            foreign_did: str,
            my_did: str,
            metadata: Optional[str]
    ):
        """"""
        pass

    @staticmethod
    @LibindyCommand('indy_list_pairwise')
    async def list_pairwise(
            wallet_handle: int
    ) -> list:
        """"""
        pass

    @staticmethod
    @LibindyCommand('indy_get_pairwise')
    async def get_pairwise(
            wallet_handle: int,
            foreign_did: str
    ) -> dict:
        """"""
        pass

    @staticmethod
    @LibindyCommand('indy_set_pairwise_metadata')
    async def set_pairwise_metadata(
            wallet_handle: int,
            foreign_did: str,
            metadata: Optional[str]
    ):
        """"""
        pass
