async def sign_and_submit_request(command_handle: int,
                                  wallet_handle: int,
                                  submitter_did: str,
                                  request_json: str,
                                  request_result_json: str) -> None:
    pass


async def submit_request(command_handle: int,
                         pool_handle: int,
                         request_json: str,
                         request_result_json: str) -> None:
    pass


async def build_get_ddo_request(command_handle: int,
                                submitter_did: str,
                                target_did: str,
                                request_json: str) -> None:
    pass


async def build_nym_request(command_handle: int,
                            submitter_did: str,
                            target_did: str,
                            verkey: str,
                            xref: str,
                            data: str,
                            role: str,
                            request_json: str) -> None:
    pass


async def build_attrib_request(command_handle: int,
                               submitter_did: str,
                               target_did: str,
                               hash: str,
                               raw: str,
                               enc: str,
                               request_json: str) -> None:
    pass


async def build_get_attrib_request(command_handle: int,
                                   submitter_did: str,
                                   target_did: str,
                                   data: str,
                                   request_json: str) -> None:
    pass


async def build_get_nym_request(command_handle: int,
                                submitter_did: str,
                                target_did: str,
                                request_json: str) -> None:
    pass


async def build_schema_request(command_handle: int,
                               submitter_did: str,
                               data: str,
                               request_json: str) -> None:
    pass


async def build_get_schema_request(command_handle: int,
                                   submitter_did: str,
                                   data: str,
                                   request_json: str) -> None:
    pass


async def build_claim_def_txn(command_handle: int,
                              submitter_did: str,
                              xref: str,
                              data: str,
                              request_result_json: str) -> None:
    pass


async def build_get_claim_def_txn(command_handle: int,
                                  submitter_did: str,
                                  xref: str,
                                  request_json: str) -> None:
    pass


async def build_node_request(command_handle: int,
                             submitter_did: str,
                             target_did: str,
                             data: str,
                             request_json: str) -> None:
    pass
