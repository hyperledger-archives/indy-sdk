from typing import Union

from .._command import LibindyCommand


class DID:

    @staticmethod
    @LibindyCommand('indy_create_and_store_my_did')
    async def create_and_store_did(
            wallet_handle: int,
            did_json: Union[dict, str]
    ) -> (str, str):
        pass

    @staticmethod
    @LibindyCommand('indy_replace_keys_start')
    async def replace_keys_start(
            wallet_handle: int,
            signing_did: str,
            did_json: Union[dict, str]
    ) -> str:
        pass

    @staticmethod
    @LibindyCommand('indy_replace_keys_apply')
    async def replace_keys_apply(
            wallet_handle: int,
            resolve_did: str
    ):
        pass

    @staticmethod
    @LibindyCommand('indy_store_their_did')
    async def store_foreign_did(
            wallet_handle: int,
            did_json: Union[dict, str]
    ):
        pass

    @staticmethod
    @LibindyCommand('indy_create_key')
    async def create_new_keys(
            wallet_handle: int,
            did_json: Union[dict, str]
    ) -> str:
        pass

    @staticmethod
    @LibindyCommand('indy_set_did_metadata')
    async def set_did_metadata(
            wallet_handle: int,
            did: str,
            metadata: str
    ):
        pass

    @staticmethod
    @LibindyCommand('indy_get_did_metadata')
    async def get_did_metadata(
            wallet_handle: int,
            did: str
    ) -> str:
        pass

    @staticmethod
    @LibindyCommand('indy_get_my_did_with_meta')
    async def get_did_with_metadata(
            wallet_handle: int,
            did: str
    ) -> dict:
        pass

    @staticmethod
    @LibindyCommand('indy_list_my_dids_with_meta')
    async def get_dids_with_metadata(
            wallet_handle: int
    ) -> list:
        pass

    @staticmethod
    @LibindyCommand('indy_set_key_metadata')
    async def set_verkey_metadata(
            wallet_handle: int,
            verkey: str,
            metadata: str
    ):
        pass

    @staticmethod
    @LibindyCommand('indy_get_key_metadata')
    async def get_verkey_metadata(
            wallet_handle: int,
            verkey: str
    ) -> str:
        pass

    @staticmethod
    @LibindyCommand('indy_key_for_did')
    async def get_did_verkey(
            pool_handle: int,
            wallet_handle: int,
            did: str
    ) -> str:
        pass

    @staticmethod
    @LibindyCommand('indy_key_for_local_did')
    async def get_local_did_verkey(
            wallet_handle: int,
            did: str
    ) -> str:
        pass

    @staticmethod
    @LibindyCommand('indy_set_endpoint_for_did')
    async def set_did_endpoint(
            wallet_handle: int,
            did: str,
            did_url: str,
            did_verkey: str
    ):
        pass

    @staticmethod
    @LibindyCommand('indy_get_endpoint_for_did')
    async def get_did_endpoint(
            wallet_handle: int,
            pool_handle: int,
            did: str
    ) -> (str, str):
        pass

    @staticmethod
    @LibindyCommand('indy_abbreviate_verkey')
    async def abbreviate_verkey(
            did: str,
            verkey: str
    ) -> str:
        pass
