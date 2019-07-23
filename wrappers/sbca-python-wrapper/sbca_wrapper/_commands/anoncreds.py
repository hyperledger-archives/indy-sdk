from ctypes import c_int32, c_uint, c_uint64
from typing import Optional, Union

from .._command import LibindyCommand


class Anoncreds:
    """A class containing all functions about credential management.
    ------------------------------------------------------------------------
    Anoncreds (Anonymous Credentials) is a collection of functions that
    define the workflows to create and manage credentials and related
    entities.
    """

    @staticmethod
    @LibindyCommand('indy_issuer_create_schema')
    async def create_schema(
            issuer_schema_did: str,
            schema_name: str,
            schema_version: str,
            schema_attributes: Union[list, str]
    ) -> (str, dict):
        """Creates a new credential schema.
        -----------------------------------------------------------------------
        Schemas describe the attribute list of a credential definition.

        Schemas are public entities and should therefore be published on the
        ledger with a SCHEMA transaction.
        -----------------------------------------------------------------------
        :param issuer_schema_did: str - A DID from the schema issuer's wallet
        :param schema_name: str - A name for the schema
        :param schema_version: str - The version of the schema
        :param schema_attributes: list, str - A list of schema attribute names
            ->  Needs to have at least 1, at most 125 entries
        -----------------------------------------------------------------------
        :returns: (
            schema_id: str - The ID of the newly created schema
            schema: dict - The newly created schema
        )
        """
        pass

    @staticmethod
    @LibindyCommand('indy_issuer_create_and_store_credential_def')
    async def create_credential_definition(
            wallet_handle: int,
            cred_def_did: str,
            cred_def_schema: Union[dict, str],
            cred_def_tag: str,
            cred_def_type: Optional[str],
            cred_def_type_config: Optional[Union[dict, str]]
    ) -> (str, dict):
        """Creates a new credential definition.
        -----------------------------------------------------------------------
        Credential definitions (CRED-DEFs) contain the credential schema,
        credential issuer DID and credential signing and revocation secrets.

        A credential definition consists of a private and a public part. The
        private part contains the signing and revocation secrets and will never
        leave the issuer wallet. The public part should be published on the
        ledger by sending a CRED_DEF request.

        The cred_def_schema NEEDS to be fetched from the ledger, as it requires
        a sequence number (seqNo) that is assigned to the schema when it is
        published.
        -----------------------------------------------------------------------
        :param wallet_handle: int - The handle to the open target wallet
        :param cred_def_did: str - A DID from the cred-def issuer's wallet
        :param cred_def_schema: dict, str - A credential schema
        :param cred_def_tag: str - A tag for the cred-def
            ->  Allows distinction between multiple credential definitions
                using the same schema
        :param cred_def_type: str - The type of the credential definition
            ->  Credential definition types define the signature and revocation
                math.
            Supported types:
                -   "CL" (Camenisch-Lysyanskaya)
            Default: "CL"
        :param cred_def_type_config: dict, str - Configurations for the
            specified credential definition type
            {
                CL: dict - The name of the credential definition type
                {
                    support_revocation: bool - Whether the credential
                        definition should support credential revocation
                        ->  Default: False
                }
            }
        -----------------------------------------------------------------------
        :returns: (
            cred_def_id: str - The ID of the newly created credential
                definition
            cred_def: dict - The public part of the newly created credential
                definition
        )
        -----------------------------------------------------------------------
        :raises CredentialDefinitionAlreadyExistsError: There already exists a
            credential definition in the target wallet that uses that DID /
            schema combination
        """
        pass

    @staticmethod
    @LibindyCommand('indy_issuer_create_and_store_revoc_reg')
    async def create_revocation_registry(
            wallet_handle: int,
            revoc_reg_did: str,
            revoc_reg_type: Optional[str],
            revoc_reg_tag: str,
            revoc_reg_cred_def_id: str,
            revoc_reg_config: Union[dict, str],
            tails_writer_handle: int
    ) -> (str, dict, dict):
        """Creates a new revocation registry for a credential definition.
        -----------------------------------------------------------------------

        -----------------------------------------------------------------------
        :param wallet_handle: int - The handle to the open target wallet
        :param revoc_reg_did: str - A DID from the revoc-reg issuer's wallet
        :param revoc_reg_type: str -
        :param revoc_reg_tag:
        :param revoc_reg_cred_def_id:
        :param revoc_reg_config:
        :param tails_writer_handle:
        -----------------------------------------------------------------------
        :returns:
        """
        pass

    @staticmethod
    @LibindyCommand('indy_issuer_create_credential_offer')
    async def create_credential_offer(
            wallet_handle: int,
            cred_def_id: str
    ) -> dict:
        """"""
        pass

    @staticmethod
    @LibindyCommand('indy_issuer_create_credential')
    async def create_credential(
            wallet_handle: int,
            cred_offer: Union[dict, str],
            cred_request: Union[dict, str],
            cred_values: Union[dict, str],
            revoc_reg_id: Optional[str],
            tails_reader_handle: Optional[int]
    ) -> (dict, Optional[str], Optional[dict]):
        """"""
        pass

    @staticmethod
    @LibindyCommand('indy_issuer_revoke_credential')
    async def revoke_credential(
            wallet_handle: int,
            tails_reader_handle: int,
            revoc_reg_id: str,
            cred_revoc_id: str
    ) -> dict:
        """"""
        pass

    @staticmethod
    @LibindyCommand('indy_issuer_merge_revocation_registry_deltas')
    async def merge_revocation_registry_deltas(
            revoc_reg_delta_1: Union[dict, str],
            revoc_reg_delta_2: Union[dict, str]
    ) -> dict:
        """"""
        pass

    @staticmethod
    @LibindyCommand('indy_prover_create_master_secret')
    async def create_master_secret(
            wallet_handle: int,
            master_secret_name: Optional[str]
    ) -> str:
        """"""
        pass

    @staticmethod
    @LibindyCommand('indy_prover_create_credential_req')
    async def create_credential_request(
            wallet_handle: int,
            cred_req_did: str,
            cred_offer: Union[dict, str],
            cred_def: Union[dict, str],
            master_secret_id: str
    ) -> (dict, dict):
        """"""
        pass

    @staticmethod
    @LibindyCommand('indy_prover_store_credential')
    async def store_credential(
            wallet_handle: int,
            cred_id: Optional[str],
            cred_req_metadata: Union[dict, str],
            cred: Union[dict, str],
            cred_def: Union[dict, str],
            revoc_reg_def: Optional[Union[dict, str]]
    ) -> str:
        """"""
        pass

    @staticmethod
    @LibindyCommand('indy_prover_get_credential')
    async def get_credential_by_id(
            wallet_handle: int,
            cred_id: str
    ) -> dict:
        """"""
        pass

    @staticmethod
    @LibindyCommand('indy_prover_get_credentials')
    async def get_credentials(
            wallet_handle: int,
            credential_filter: Union[dict, str]
    ) -> list:
        """"""
        pass

    @staticmethod
    @LibindyCommand('indy_prover_get_credentials_for_proof_req')
    async def fetch_credentials_for_proof_request(
            wallet_handle: int,
            proof_req: Union[dict, str]
    ) -> dict:
        """"""
        pass

    @staticmethod
    @LibindyCommand('indy_prover_search_credentials',
                    return_type=(c_int32, c_uint))
    async def open_credential_search(
            wallet_handle: int,
            cred_search_queries: Union[dict, str]
    ) -> (int, int):
        """"""
        pass

    @staticmethod
    @LibindyCommand('indy_prover_fetch_credentials',
                    cred_count=lambda arg: c_uint(arg))
    async def get_credentials_from_search(
            search_handle: int,
            cred_count: int
    ) -> list:
        """"""
        pass

    @staticmethod
    @LibindyCommand('indy_prover_close_credentials_search')
    async def close_credential_search(
            search_handle: int
    ):
        """"""
        pass

    @staticmethod
    @LibindyCommand('indy_prover_search_credentials_for_proof_req')
    async def open_proof_request_search(
            wallet_handle: int,
            proof_req: Union[dict, str],
            cred_search_queries: Union[dict, str]
    ) -> int:
        """"""
        pass

    @staticmethod
    @LibindyCommand('indy_prover_fetch_credentials_for_proof_req',
                    cred_count=lambda arg: c_uint(arg))
    async def get_credentials_from_proof_request_search(
            search_handle: int,
            item_id: str,
            cred_count: int
    ) -> list:
        """"""
        pass

    @staticmethod
    @LibindyCommand('indy_prover_close_credentials_search_for_proof_req')
    async def close_proof_request_search(
            search_handle: int
    ):
        """"""
        pass

    @staticmethod
    @LibindyCommand('indy_prover_create_proof')
    async def create_proof(
            wallet_handle: int,
            proof_req: Union[dict, str],
            proof_creds: Union[dict, str],
            master_secret_name: str,
            proof_schemas: Union[dict, str],
            proof_cred_defs: Union[dict, str],
            proof_revoc_states: Union[dict, str]
    ) -> dict:
        """"""
        pass

    @staticmethod
    @LibindyCommand('indy_verifier_verify_proof')
    async def verify_proof(
            proof_req: Union[dict, str],
            proof: Union[dict, str],
            proof_schemas: Union[dict, str],
            proof_cred_defs: Union[dict, str],
            proof_revoc_reg_defs: Union[dict, str],
            proof_revoc_regs: Union[dict, str]
    ) -> bool:
        """"""
        pass

    @staticmethod
    @LibindyCommand('indy_create_revocation_state',
                    timestamp=lambda arg: c_uint64(arg))
    async def create_revocation_state(
            tails_reader_handle: int,
            revoc_reg_defs: Union[dict, str],
            revoc_reg_delta: Union[dict, str],
            timestamp: int,
            cred_revoc_id: str
    ) -> dict:
        """"""
        pass

    @staticmethod
    @LibindyCommand('indy_update_revocation_state',
                    timestamp=lambda arg: c_uint64(arg))
    async def update_revocation_state(
            tails_reader_handle: int,
            revoc_state: Union[dict, str],
            revoc_reg_def: Union[dict, str],
            revoc_reg_delta: Union[dict, str],
            timestamp: int,
            cred_revoc_id: str
    ) -> dict:
        """"""
        pass
