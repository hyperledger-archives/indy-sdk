async def sign_and_submit_request(pool_handle: int,
                                  wallet_handle: int,
                                  submitter_did: str,
                                  request_json: str) -> str:
    pass


async def submit_request(pool_handle: int,
                         request_json: str) -> str:
    pass


async def build_get_ddo_request(submitter_did: str,
                                target_did: str) -> str:
    pass


async def build_nym_request(submitter_did: str,
                            target_did: str,
                            verkey: str,
                            alias: str,
                            role: str) -> str:
    pass


async def build_attrib_request(submitter_did: str,
                               target_did: str,
                               hash: str,
                               raw: str,
                               enc: str) -> str:
    pass


async def build_get_attrib_request(submitter_did: str,
                                   target_did: str,
                                   data: str) -> str:
    pass


async def build_get_nym_request(submitter_did: str,
                                target_did: str) -> str:
    pass


async def build_schema_request(submitter_did: str,
                               data: str) -> str:
    pass


async def build_get_schema_request(submitter_did: str,
                                   dest: str,
                                   data: str) -> str:
    pass


async def build_claim_def_txn(submitter_did: str,
                              xref: int,
                              signature_type: str,
                              data: str) -> str:
    pass


async def build_get_claim_def_txn(submitter_did: str,
                                  xref: int,
                                  signature_type: str,
                                  origin: str) -> str:
    pass


async def build_node_request(submitter_did: str,
                             target_did: str,
                             data: str) -> str:
    pass


async def build_get_txn_request(submitter_did: str,
                                data: str) -> str:
    pass
