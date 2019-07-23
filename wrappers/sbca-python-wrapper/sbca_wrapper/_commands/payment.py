from typing import Optional, Union

from .._command import LibindyCommand


class Payment:

    @staticmethod
    @LibindyCommand('indy_create_payment_address')
    async def create_payment_address(
            wallet_handle: int,
            payment_method: str,
            address_config: Union[dict, str]
    ) -> str:
        """"""
        pass

    @staticmethod
    @LibindyCommand('indy_list_payment_addresses')
    async def list_payment_addresses(
            wallet_handle: int
    ) -> list:
        """"""
        pass

    @staticmethod
    @LibindyCommand('indy_add_request_fees')
    async def add_request_fees(
            wallet_handle: int,
            sender_did: Optional[str],
            request: Union[dict, str],
            inputs: Union[list, str],
            outputs: Union[list, str],
            additional_info: Optional[str]
    ) -> (dict, str):
        """"""
        pass

    @staticmethod
    @LibindyCommand('indy_parse_response_with_fees')
    async def parse_response_with_fees(
            payment_method: str,
            response: Union[dict, str]
    ) -> dict:
        """"""
        pass

    @staticmethod
    @LibindyCommand('indy_build_get_payment_sources_request')
    async def build_get_payment_sources_request(
            wallet_handle: int,
            sender_did: Optional[str],
            payment_address: str
    ) -> (dict, str):
        """"""
        pass

    @staticmethod
    @LibindyCommand('indy_parse_get_payment_sources_response')
    async def parse_get_payment_sources_response(
            payment_method: str,
            response_raw: Union[dict, str]
    ) -> list:
        """"""
        pass

    @staticmethod
    @LibindyCommand('indy_build_payment_req')
    async def build_payment_request(
            wallet_handle: int,
            sender_did: Optional[str],
            inputs: Union[list, str],
            outputs: Union[list, str],
            additional_info: Optional[str]
    ) -> (dict, str):
        """"""
        pass

    @staticmethod
    @LibindyCommand('indy_parse_payment_response')
    async def parse_payment_response(
            payment_method: str,
            response_raw: Union[dict, str]
    ) -> list:
        """"""
        pass

    @staticmethod
    @LibindyCommand('indy_build_mint_req')
    async def build_mint_request(
            wallet_handle: int,
            sender_did: Optional[str],
            outputs: Union[list, str],
            additional_info: Optional[str]
    ) -> (dict, str):
        """"""
        pass

    @staticmethod
    @LibindyCommand('indy_build_set_txn_fees_req')
    async def build_set_transaction_fees_request(
            wallet_handle: int,
            sender_did: Optional[int],
            payment_method: str,
            transaction_fees: Union[dict, str]
    ) -> dict:
        """"""
        pass

    @staticmethod
    @LibindyCommand('indy_build_get_txn_fees_req')
    async def build_get_transaction_fees_request(
            wallet_handle: int,
            sender_did: Optional[str],
            payment_method: str
    ) -> dict:
        """"""
        pass

    @staticmethod
    @LibindyCommand('indy_parse_get_txn_fees_response')
    async def parse_get_transaction_fees_response(
            payment_method: str,
            response_raw: Union[dict, str]
    ) -> dict:
        """"""
        pass

    @staticmethod
    @LibindyCommand('indy_build_verify_payment_req')
    async def build_verify_payment_request(
            wallet_handle: int,
            sender_did: Optional[str],
            receipt: str
    ) -> (dict, str):
        """"""
        pass

    @staticmethod
    @LibindyCommand('indy_parse_verify_payment_response')
    async def parse_verify_payment_response(
            payment_method: str,
            response_raw: Union[dict, str]
    ) -> dict:
        """"""
        pass
