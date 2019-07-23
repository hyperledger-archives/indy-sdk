from ctypes import c_int64
from typing import Optional, Union

from .._command import LibindyCommand


class Ledger:

    @staticmethod
    @LibindyCommand('indy_sign_request')
    async def sign_request(
            wallet_handle: int,
            signing_did: str,
            request: Union[dict, str]
    ) -> dict:
        """"""
        pass

    @staticmethod
    @LibindyCommand('indy_multi_sign_request')
    async def multi_sign_request(
            wallet_handle: int,
            signing_did: str,
            request: Union[dict, str]
    ) -> dict:
        """"""
        pass

    @staticmethod
    @LibindyCommand('indy_submit_request')
    async def submit_request(
            pool_handle: int,
            request: Union[dict, str]
    ) -> dict:
        """"""
        pass

    @staticmethod
    @LibindyCommand('indy_submit_action')
    async def submit_action(
            pool_handle: int,
            request: Union[dict, str],
            action_nodes: Union[list, str],
            action_nodes_timeout: int
    ) -> dict:
        """"""
        pass

    @staticmethod
    @LibindyCommand('indy_sign_and_submit_request')
    async def sign_and_submit_request(
            pool_handle: int,
            wallet_handle: int,
            signing_did: str,
            request: Union[dict, str]
    ) -> dict:
        """"""
        pass

    @staticmethod
    @LibindyCommand('indy_build_attrib_request')
    async def attrib_request(
            sender_did: str,
            target_did: str,
            attrib_hash: Optional[str],
            attrib_raw: Optional[str],
            attrib_encoded: Optional[str]
    ) -> dict:
        """"""
        pass

    @staticmethod
    @LibindyCommand('indy_build_get_attrib_request')
    async def get_attrib_request(
            sender_did: Optional[str],
            target_did: str, attrib_raw:
            Optional[str],
            attrib_hash: Optional[str],
            attrib_encoded: Optional[str]
    ) -> dict:
        """"""
        pass

    @staticmethod
    @LibindyCommand('indy_build_cred_def_request')
    async def cred_def_request(
            sender_did: str,
            cred_def_data: Union[dict, str]
    ) -> dict:
        """"""
        pass

    @staticmethod
    @LibindyCommand('indy_build_get_cred_def_request')
    async def get_cred_def_request(
            sender_did: Optional[str],
            cred_def_id: str
    ) -> dict:
        """"""
        pass

    @staticmethod
    @LibindyCommand('indy_parse_get_cred_def_response')
    async def parse_get_cred_def_response(
            response_raw: Union[dict, str]
    ) -> (str, dict):
        """"""
        pass

    @staticmethod
    @LibindyCommand('indy_build_get_ddo_request')
    async def get_ddo_request(
            sender_did: Optional[str],
            target_did: str
    ) -> dict:
        """"""
        pass

    @staticmethod
    @LibindyCommand('indy_build_node_request')
    async def node_request(
            sender_did: str,
            node_did: str,
            node_data: Union[dict, str]
    ) -> dict:
        """"""
        pass

    @staticmethod
    @LibindyCommand('indy_build_nym_request')
    async def nym_request(
            sender_did: str,
            target_did: str,
            target_verkey: Optional[str],
            target_alias: Optional[str],
            target_role: Optional[str]
    ) -> dict:
        """"""
        pass

    @staticmethod
    @LibindyCommand('indy_build_get_nym_request')
    async def get_nym_request(
            sender_did: Optional[str],
            target_did: str
    ) -> dict:
        """"""
        pass

    @staticmethod
    @LibindyCommand('indy_build_pool_config_request')
    async def pool_config_request(
            sender_did: str,
            pool_can_write: bool,
            request_force: bool
    ) -> dict:
        """"""
        pass

    @staticmethod
    @LibindyCommand('indy_build_pool_restart_request')
    async def pool_restart_request(
            sender_did: str,
            request_action: str,
            restart_datetime: str
    ) -> dict:
        """"""
        pass

    @staticmethod
    @LibindyCommand('indy_build_pool_upgrade_request')
    async def pool_upgrade_request(
            sender_did: str,
            upgrade_name: str,
            upgrade_package_version: str,
            request_action: str,
            upgrade_package_hash: str,
            upgrade_node_timeout: Optional[int],
            upgrade_node_schedule: Optional[Union[dict, str]],
            upgrade_justification: Optional[str],
            upgrade_package_reinstall: bool,
            request_force: bool,
            upgrade_package_name: Optional[str]
    ) -> dict:
        """"""
        pass

    @staticmethod
    @LibindyCommand('indy_build_get_revoc_reg_request',
                    timestamp=lambda arg: c_int64(arg))
    async def get_revoc_reg_request(
            sender_did: Optional[str],
            revoc_reg_def_id: str,
            timestamp: int
    ) -> dict:
        """"""
        pass

    @staticmethod
    @LibindyCommand('indy_parse_get_revoc_reg_response')
    async def parse_get_revoc_reg_response(
            response_raw: Union[dict, str]
    ) -> (str, dict, int):
        """"""
        pass

    @staticmethod
    @LibindyCommand('indy_build_revoc_reg_def_request')
    async def revoc_reg_def_request(
            sender_did: str,
            revoc_reg_data: Union[dict, str]
    ) -> dict:
        """"""
        pass

    @staticmethod
    @LibindyCommand('indy_build_get_revoc_reg_def_request')
    async def get_revoc_reg_def_request(
            sender_did: Optional[str],
            revoc_reg_def_id: str
    ) -> dict:
        """"""
        pass

    @staticmethod
    @LibindyCommand('indy_parse_get_revoc_reg_def_response')
    async def parse_get_revoc_reg_def_response(
            response_raw: Union[dict, str]
    ) -> (str, dict):
        """"""
        pass

    @staticmethod
    @LibindyCommand('indy_build_revoc_reg_entry_request')
    async def revoc_reg_entry_request(
            sender_did: str,
            revoc_reg_def_id: str,
            revoc_reg_type: str,
            entry_value: Union[dict, str]
    ) -> dict:
        """"""
        pass

    @staticmethod
    @LibindyCommand('indy_build_get_revoc_reg_delta_request',
                    delta_from=lambda arg: c_int64(arg) if arg else -1,
                    delta_to=lambda arg: c_int64(arg))
    async def get_revoc_reg_delta_request(
            sender_did: Optional[str],
            revoc_reg_def_id: str,
            delta_from: Optional[int],
            delta_to: int
    ) -> dict:
        """"""
        pass

    @staticmethod
    @LibindyCommand('indy_parse_get_revoc_reg_delta_response')
    async def parse_get_revoc_reg_delta_response(
            response_raw: Union[dict, str]
    ) -> (str, dict, int):
        """"""
        pass

    @staticmethod
    @LibindyCommand('indy_build_schema_request')
    async def schema_request(
            sender_did: str,
            schema_data: Union[dict, str]
    ) -> dict:
        """"""
        pass

    @staticmethod
    @LibindyCommand('indy_build_get_schema_request')
    async def get_schema_request(
            sender_did: Optional[str],
            schema_id: str
    ) -> dict:
        """"""
        pass

    @staticmethod
    @LibindyCommand('indy_parse_get_schema_response')
    async def parse_get_schema_response(
            response_raw: Union[dict, str]
    ) -> (str, dict):
        """"""
        pass

    @staticmethod
    @LibindyCommand('indy_build_get_txn_request')
    async def get_txn_request(
            sender_did: Optional[str],
            ledger_type: Optional[str],
            transaction_seq_no: int
    ) -> dict:
        """"""
        pass

    @staticmethod
    @LibindyCommand('indy_build_get_validator_info_request')
    async def get_validator_info_request(
            sender_did: str
    ) -> dict:
        """"""
        pass

    @staticmethod
    @LibindyCommand('indy_get_response_metadata')
    async def get_response_metadata(
            response: Union[dict, str]
    ) -> dict:
        """"""
        pass
