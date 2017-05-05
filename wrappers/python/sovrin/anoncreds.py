from typing import Callable
from . import SovrinError

class Anoncreds(object):

    """TODO: document it"""

    async def issuer_create_and_store_claim_def(wallet_handle: int,
                                                schema_json: str,
                                                signature_type: str,
                                                create_non_revoc: bool,
                                                claim_def_json: str,
                                                claim_def_uuid: str) -> None:
        pass

    async def issuer_create_and_store_revoc_reg(wallet_handle: int,
                                                claim_def_seq_no: int,
                                                max_claim_num: int,
                                                revoc_reg_json: str,
                                                revoc_reg_uuid: str) -> None:
        pass

    async def issuer_create_claim(wallet_handle: int,
                                  claim_req_json: str,
                                  claim_json: str,
                                  revoc_reg_seq_no: int,
                                  user_revoc_index: int,
                                  revoc_reg_update_json: str,
                                  xclaim_json: str) -> None:
        pass

    async def issuer_revoke_claim(wallet_handle: int,
                                  claim_def_seq_no: int,
                                  revoc_reg_seq_no: int,
                                  user_revoc_index: int,
                                  revoc_reg_update_json: str) -> None:
        pass

    async def prover_store_claim_offer(wallet_handle: int,
                                       claim_offer_json: str) -> None:
        pass

    async def prover_get_claim_offers(wallet_handle: int,
                                      filter_json: str,
                                      claim_offers_json: str) -> None:
        pass

    async def prover_create_master_secret(wallet_handle: int,
                                          master_secret_name: str) -> None:
        pass

    async def prover_create_and_store_claim_req(wallet_handle: int,
                                                prover_did: str,
                                                claim_offer_json: str,
                                                claim_def_json: str,
                                                master_secret_name: str,
                                                claim_req_json: str) -> None:
        pass

    async def prover_store_claim(wallet_handle: int,
                                 claims_json: str) -> None:
        pass

    async def prover_get_claims(wallet_handle: int,
                                filter_json: str,
                                claims_json: str) -> None:
        pass

    async def prover_get_claims_for_proof_req(wallet_handle: int,
                                              proof_request_json: str,
                                              claims_json: str) -> None:
        pass

    async def prover_create_proof(wallet_handle: int,
                                  proof_req_json: str,
                                  requested_claims_json: str,
                                  schemas_json: str,
                                  claim_defs_json: str,
                                  revoc_regs_json: str,
                                  proof_json: str) -> None:
        pass

    async def verifier_verify_proof(wallet_handle: int,
                                    proof_request_json: str,
                                    proof_json: str,
                                    schemas_json: str,
                                    claim_defs_jsons: str,
                                    revoc_regs_json: str,
                                    valid: bool) -> None:
        pass
