import json

import pytest
import time

from enum import IntEnum
from pprint import pformat

from indy import ledger, anoncreds, wallet, blob_storage

from tests.ledger.test_submit_request import ensure_previous_request_applied

import logging


# logging.getLogger("tests").setLevel(logging.ERROR)
# logging.getLogger("indy").setLevel(logging.ERROR)


def ppjson(dumpit, elide_to: int = None) -> str:
    if elide_to is not None:
        elide_to = max(elide_to, 3)  # make room for ellipses '...'
    try:
        rv = json.dumps(json.loads(dumpit) if isinstance(dumpit, str) else dumpit, indent=4)
    except TypeError:
        rv = '{}'.format(pformat(dumpit, indent=4, width=120))
    return rv if elide_to is None or len(rv) <= elide_to else '{}...'.format(rv[0:(elide_to - 3)])


class Ink(IntEnum):
    BLACK = 30
    RED = 31
    GREEN = 32
    YELLOW = 33
    BLUE = 34
    MAGENTA = 35
    CYAN = 36
    WHITE = 37


    def __call__(self, message: str) -> str:
        return '\033[{}m{}\033[0m'.format(self.value, message)


@pytest.mark.asyncio
async def test_anoncreds_revocation_interaction_test_issuance_by_demand_4_creds(pool_name, pool_handle, wallet_handle,
                                                                                identity_my, identity_my1, path_home,
                                                                                did_my2, credentials):
    issuer_did, _ = identity_my
    issuer_wallet_handle = wallet_handle

    prover_did, _ = identity_my1

    #  Prover Creates Wallet and Get Wallet Handle
    prover_wallet_config = '{"id":"prover_wallet"}'
    await wallet.create_wallet(prover_wallet_config, credentials)
    prover_wallet_handle = await wallet.open_wallet(prover_wallet_config, credentials)

    # Issuer Creates Schema
    (schema_id, schema_json) = await anoncreds.issuer_create_schema(
        issuer_did,
        "gvt",
        "1.0",
        json.dumps(["name", "age", "sex", "height"])
    )

    # Issuer Posts Schema
    schema_request = await ledger.build_schema_request(issuer_did, schema_json)
    await ledger.sign_and_submit_request(pool_handle, issuer_wallet_handle, issuer_did, schema_request)

    # Issuer Gets Schema from Ledger
    get_schema_request = await ledger.build_get_schema_request(issuer_did, str(schema_id))
    get_schema_response = await ensure_previous_request_applied(
        pool_handle,
        get_schema_request,
        lambda response: response['result']['seqNo'] is not None
    )
    (schema_id, schema_json) = await ledger.parse_get_schema_response(get_schema_response)

    #  Issuer Creates credential Definition for Schema
    (cred_def_id, cred_def_json) = await anoncreds.issuer_create_and_store_credential_def(
        issuer_wallet_handle,
        issuer_did,
        schema_json,
        'tag1',
        'CL',
        '{"support_revocation": true}'
    )

    # Issuer Posts Credential Definition
    cred_def_request = await ledger.build_cred_def_request(issuer_did, cred_def_json)
    await ledger.sign_and_submit_request(pool_handle, issuer_wallet_handle, issuer_did, cred_def_request)

    #  Issuer Creates Revocation Registry
    tails_writer_config = json.dumps({'base_dir': str(path_home.joinpath("tails")), 'uri_pattern': ''})
    tails_writer = await blob_storage.open_writer('default', tails_writer_config)
    (rev_reg_def_id, rev_reg_def_json, rev_reg_entry_json) = await anoncreds.issuer_create_and_store_revoc_reg(
        issuer_wallet_handle,
        issuer_did, None,
        'tag1',
        cred_def_id,
        '{"max_cred_num": 16, "issuance_type":"ISSUANCE_ON_DEMAND"}',
        tails_writer
    )

    # Issuer posts Revocation Registry Definition to Ledger
    revoc_reg_request = await ledger.build_revoc_reg_def_request(issuer_did, rev_reg_def_json)
    await ledger.sign_and_submit_request(pool_handle, issuer_wallet_handle, issuer_did, revoc_reg_request)

    # Issuer posts Revocation Registry Entry to Ledger
    revoc_reg_entry_request = await ledger.build_revoc_reg_entry_request(
        issuer_did,
        rev_reg_def_id,
        "CL_ACCUM",
        rev_reg_entry_json
    )
    await ledger.sign_and_submit_request(pool_handle, issuer_wallet_handle, issuer_did, revoc_reg_entry_request)

    # ISSUANCE Credential for Prover

    # Prover Creates Master Secret
    master_secret_id = "master_secret"
    await anoncreds.prover_create_master_secret(prover_wallet_handle, master_secret_id)

    #  Issuer Creates credential Offer
    cred_offer_json = await anoncreds.issuer_create_credential_offer(issuer_wallet_handle, cred_def_id)
    cred_offer = json.loads(cred_offer_json)

    # Prover Gets Credential Definition from Ledger
    get_cred_def_request = await ledger.build_get_cred_def_request(prover_did, cred_offer['cred_def_id'])
    get_cred_def_response = await ensure_previous_request_applied(
        pool_handle,
        get_cred_def_request,
        lambda response: response['result']['seqNo'] is not None
    )
    (cred_def_id, cred_def_json) = await ledger.parse_get_cred_def_response(get_cred_def_response)

    #  Prover create credential Request
    (cred_req_json, cred_req_metadata_json) = await anoncreds.prover_create_credential_req(
        prover_wallet_handle,
        prover_did,
        cred_offer_json,
        cred_def_json,
        master_secret_id
    )

    #  Issuer Opens Tails reader
    blob_storage_reader_cfg_handle = await blob_storage.open_reader('default', tails_writer_config)

    #  Issuer create credential for credential Request
    #  note that encoding is not standardized by Indy except that 32-bit integers are encoded as themselves. IS-786
    cred_values_json = [
        json.dumps(
            {
                "sex": {"raw": "male", "encoded": "7239210949258394887428692050081607692519917050011144233115103"},
                "name": {"raw": "Alex", "encoded": "1139481716457488690172217916278103335"},
                "height": {"raw": "180", "encoded": "180"},
                "age": {"raw": "28", "encoded": "28"}
            }
        ),
        json.dumps(
            {
                "sex": {"raw": "female", "encoded": "7239210949258394887428692050081607692519917050011144233115104"},
                "name": {"raw": "Ludmilla", "encoded": "690172217916278103335"},
                "height": {"raw": "171", "encoded": "171"},
                "age": {"raw": "30", "encoded": "30"}
            }
        ),
        json.dumps(
            {
                "sex": {"raw": "male", "encoded": "7239210949258394887428692050081607692519917050011144233115103"},
                "name": {"raw": "Boris", "encoded": "239572896838295261729"},
                "height": {"raw": "172", "encoded": "172"},
                "age": {"raw": "48", "encoded": "48"}
            }
        ),
        json.dumps(
            {
                "sex": {"raw": "female", "encoded": "7239210949258394887428692050081607692519917050011144233115104"},
                "name": {"raw": "Olga", "encoded": "258623589235632895688"},
                "height": {"raw": "173", "encoded": "173"},
                "age": {"raw": "78", "encoded": "78"}
            }
        )
    ]

    cred_json = [None] * 4
    cred_rev_id = [None] * 4
    rev_reg_delta_json = [None] * 4
    credential = [None] * 4
    print(Ink.CYAN('\n\nCreating and storing credentials'))
    for i in range(4):
        (cred_json[i], cred_rev_id[i], rev_reg_delta_json[i]) = await anoncreds.issuer_create_credential(
            issuer_wallet_handle,
            cred_offer_json,
            cred_req_json,
            cred_values_json[i],
            rev_reg_def_id,
            blob_storage_reader_cfg_handle
        )

        # Issuer Posts Revocation Registry Delta to Ledger
        revoc_reg_entry_request = await ledger.build_revoc_reg_entry_request(
            issuer_did,
            rev_reg_def_id,
            "CL_ACCUM",
            rev_reg_delta_json[i]
        )
        await ledger.sign_and_submit_request(pool_handle, issuer_wallet_handle, issuer_did, revoc_reg_entry_request)

        # Prover Gets RevocationRegistryDefinition
        credential[i] = json.loads(cred_json[i])
        get_revoc_reg_def_request = await ledger.build_get_revoc_reg_def_request(
            prover_did,
            credential[i]['rev_reg_id']
        )
        get_revoc_reg_def_response = await ensure_previous_request_applied(
            pool_handle,
            get_revoc_reg_def_request,
            lambda response: response['result']['seqNo'] is not None
        )
        (rev_reg_id, revoc_reg_def_json) = await ledger.parse_get_revoc_reg_def_response(get_revoc_reg_def_response)

        print(
            '.. Prover storing credential {}, rev-id {}, values {}'.format(
                'cred_%s_id' % i,
                cred_rev_id[i],
                ppjson(json.loads(cred_json[i])['values'])
            )
        )
        # Prover Stores Credential
        await anoncreds.prover_store_credential(
            prover_wallet_handle,
            'cred_%s_id' % i,
            cred_req_metadata_json,
            cred_json[i],
            cred_def_json,
            revoc_reg_def_json
        )

    # VERIFYING Prover Credentials (Alex, Olga) when issuer has not revoked any
    time.sleep(2)
    to = int(time.time())

    proof_req_json = {
        'Alex': json.dumps(
            {
                'nonce': '100000000001',
                'name': 'proof_req',
                'version': '0.1',
                'requested_attributes': {
                    'attr1_referent': {'name': 'name'}
                },
                'requested_predicates': {
                    'pred1_referent': {'name': 'height', 'p_type': '>=', 'p_value': 180}
                },
                "non_revoked": {"to": to}
            }
        ),
        'Olga': json.dumps(
            {
                'nonce': '100000000000',
                'name': 'proof_req',
                'version': '0.1',
                'requested_attributes': {
                    'attr1_referent': {'name': 'name'}
                },
                'requested_predicates': {
                    'pred1_referent': {'name': 'age', 'p_type': '>=', 'p_value': 65}
                },
                "non_revoked": {"to": to}
            }
        )
    }

    rev_states = {
        'Alex': {
            'timestamp': 0,
            'value': ''
        },
        'Olga': {
            'timestamp': 0,
            'value': ''
        }
    }

    cred_info = {}
    print(Ink.CYAN('\n\nCreating proofs'))
    for proof_name in proof_req_json:
        print('\n\n.. Creating proof for {}'.format(proof_name))
        # Prover Gets credentials for Proof Request
        search_credentials_for_proof_handle = await anoncreds.prover_search_credentials_for_proof_req(
            prover_wallet_handle,
            proof_req_json[proof_name],
            None
        )
        fetched_credential_json = await anoncreds.prover_fetch_credentials_for_proof_req(
            search_credentials_for_proof_handle,
            'attr1_referent',
            4
        )
        await anoncreds.prover_close_credentials_search_for_proof_req(search_credentials_for_proof_handle)

        pred_spec = None
        for i in range(4):
            found = json.loads(fetched_credential_json)[i]['cred_info']  # don't clobber cred_info ...
            pred_spec = json.loads(proof_req_json[proof_name])['requested_predicates']['pred1_referent']
            if found['attrs'][pred_spec['name']] >= str(pred_spec['p_value']):
                break
        else:
            print('!! No matching credential found for {}'.format(proof_name))
            assert False
        cred_info[proof_name] = found  # ... until we have the right cred info
        print(
            '.. Credential info for proof on {}>={} ({}): {}'.format(
                pred_spec['name'],
                pred_spec['p_value'],
                proof_name,
                ppjson(cred_info[proof_name])
            )
        )

        # Prover Gets RevocationRegistryDelta from Ledger
        get_revoc_reg_delta_request = await ledger.build_get_revoc_reg_delta_request(
            prover_did,
            rev_reg_def_id,
            None,
            to
        )
        get_revoc_reg_delta_response = await ensure_previous_request_applied(
            pool_handle,
            get_revoc_reg_delta_request,
            lambda response: response['result']['seqNo'] is not None
        )
        (rev_reg_id, revoc_reg_delta_json, timestamp) = await ledger.parse_get_revoc_reg_delta_response(
            get_revoc_reg_delta_response
        )

        # Prover Creates Revocation State
        rev_state_from_timestamp_to_now_json = await anoncreds.create_revocation_state(
            blob_storage_reader_cfg_handle,
            revoc_reg_def_json,
            revoc_reg_delta_json,
            timestamp,
            cred_info[proof_name]['cred_rev_id']
        )
        rev_states[proof_name] = {
            'timestamp': timestamp,
            'value': rev_state_from_timestamp_to_now_json,
        }

        # Prover Gets Schema from Ledger
        get_schema_request = await ledger.build_get_schema_request(prover_did, str(cred_info[proof_name]["schema_id"]))
        get_schema_response = await ensure_previous_request_applied(
            pool_handle,
            get_schema_request,
            lambda response: response['result']['seqNo'] is not None
        )
        (schema_id, schema_json) = await ledger.parse_get_schema_response(get_schema_response)

        # Prover Creates Proof for Proof Request
        requested_credentials_json = json.dumps(
            {
                'self_attested_attributes': {},
                'requested_attributes': {
                    'attr1_referent': {
                        'cred_id': cred_info[proof_name]['referent'],
                        'revealed': True,
                        'timestamp': timestamp
                    }
                },
                'requested_predicates': {
                    'pred1_referent': {
                        'cred_id': cred_info[proof_name]['referent'],
                        'timestamp': timestamp
                    }
                }
            }
        )

        schemas_json = json.dumps({schema_id: json.loads(schema_json)})
        credential_defs_json = json.dumps({cred_def_id: json.loads(cred_def_json)})
        revoc_states_json = json.dumps({rev_reg_id: {timestamp: json.loads(rev_state_from_timestamp_to_now_json)}})

        proof_json = await anoncreds.prover_create_proof(
            prover_wallet_handle,
            proof_req_json[proof_name],
            requested_credentials_json,
            master_secret_id,
            schemas_json,
            credential_defs_json,
            revoc_states_json)
        proof = json.loads(proof_json)
        print('.. Proof for {}: {}'.format(proof_name, ppjson(proof_json, 1024)))

        # Verifier Gets required entities from Ledger
        verifier_did = did_my2
        identifier = proof['identifiers'][0]

        # Verifier Gets Schema from Ledger
        get_schema_request = await ledger.build_get_schema_request(verifier_did, identifier['schema_id'])
        get_schema_response = await ensure_previous_request_applied(
            pool_handle,
            get_schema_request,
            lambda response: response['result']['seqNo'] is not None
        )
        (schema_id, schema_json) = await ledger.parse_get_schema_response(get_schema_response)

        # Verifier Gets Credential Definition from Ledger
        get_cred_def_request = await ledger.build_get_cred_def_request(verifier_did, identifier['cred_def_id'])
        get_cred_def_response = await ensure_previous_request_applied(
            pool_handle,
            get_cred_def_request,
            lambda response: response['result']['seqNo'] is not None
        )
        (cred_def_id, cred_def_json) = await ledger.parse_get_cred_def_response(get_cred_def_response)

        # Verifier Gets Revocation Registry Definition from Ledger
        get_revoc_reg_def_request = await ledger.build_get_revoc_reg_def_request(verifier_did, identifier['rev_reg_id'])
        get_revoc_reg_def_response = await ensure_previous_request_applied(
            pool_handle,
            get_revoc_reg_def_request,
            lambda response: response['result']['seqNo'] is not None
        )
        (rev_reg_id, revoc_reg_def_json) = await ledger.parse_get_revoc_reg_def_response(get_revoc_reg_def_response)

        # Verifier Gets Revocation Registry from Ledger
        get_revoc_reg_request = await ledger.build_get_revoc_reg_request(
            verifier_did,
            identifier['rev_reg_id'],
            identifier['timestamp']
        )
        get_revoc_reg_response = await ensure_previous_request_applied(
            pool_handle,
            get_revoc_reg_request,
            lambda response: response['result']['seqNo'] is not None
        )
        (rev_reg_id, rev_reg_json, identifier) = await ledger.parse_get_revoc_reg_response(get_revoc_reg_response)

        # Verifier verify proof
        assert proof_name == proof['requested_proof']['revealed_attrs']['attr1_referent']['raw']

        schemas_json = json.dumps({schema_id: json.loads(schema_json)})
        credential_defs_json = json.dumps({cred_def_id: json.loads(cred_def_json)})
        revoc_ref_defs_json = json.dumps({rev_reg_id: json.loads(revoc_reg_def_json)})
        revoc_regs_json = json.dumps({rev_reg_id: {timestamp: json.loads(rev_reg_json)}})

        print(Ink.RED('>> rev regs: {}'.format(ppjson(revoc_regs_json))))
        verified = await anoncreds.verifier_verify_proof(
            proof_req_json[proof_name],
            proof_json,
            schemas_json,
            credential_defs_json,
            revoc_ref_defs_json,
            revoc_regs_json
        )
        print(Ink.GREEN('.. Proof for {} before any revocations verifies as {}'.format(proof_name, verified)))
        assert verified

    proof_req_expected_result = {
        'Alex': False,
        'Olga': True
    }
    # Issuer revokes creds for all but Olga, one by one, creating and verifying proofs for Alex and Olga each time
    print(Ink.CYAN('\n\nIssuer revoking creds one by one'))
    for i in range(3):
        revo_name = json.loads(cred_values_json[i])['name']['raw']
        print(
            '\n\n.. Issuer revoking {} (cred rev id {}) at epoch={}'.format(
                revo_name,
                cred_rev_id[i],
                int(time.time())
            )
        )
        rev_reg_delta_json = await anoncreds.issuer_revoke_credential(
            issuer_wallet_handle,
            blob_storage_reader_cfg_handle,
            rev_reg_def_id,
            cred_rev_id[i]
        )

        # Issuer Posts RevocationRegistryDelta to Ledger
        revoc_reg_entry_request = await ledger.build_revoc_reg_entry_request(
            issuer_did,
            rev_reg_def_id,
            "CL_ACCUM",
            rev_reg_delta_json
        )
        await ledger.sign_and_submit_request(pool_handle, issuer_wallet_handle, issuer_did, revoc_reg_entry_request)

        print(Ink.CYAN('.. Processing proofs after revocation of {}'.format(revo_name)))
        for proof_name in proof_req_json:
            #  VERIFYING Prover Credential after Revocation
            time.sleep(2)
            fro = to
            to = int(time.time())
            print(
                '.. .. after sleeping 2 seconds, doing {} for interval [{}, {}]'.format(
                    proof_name,
                    fro,
                    to
                )
            )
            #  There two possible ways:

            #  1) Prover creates updates previously created revocation state
            # Prover Gets RevocationRegistryDelta from Ledger
            #  from: when last prover revocation state were created
            #  to: to
            get_revoc_reg_delta_request = await ledger.build_get_revoc_reg_delta_request(
                prover_did,
                rev_reg_def_id,
                rev_states[proof_name]['timestamp'],
                to
            )

            get_revoc_reg_delta_response = await ensure_previous_request_applied(
                pool_handle,
                get_revoc_reg_delta_request,
                lambda response: response['result']['seqNo'] is not None
            )
            (rev_reg_id, revoc_reg_delta_json, timestamp) = await ledger.parse_get_revoc_reg_delta_response(
                get_revoc_reg_delta_response
            )

            # Prover Updates previously created Revocation State
            rev_state_from_timestamp_to_now_json = await anoncreds.update_revocation_state(
                blob_storage_reader_cfg_handle,
                rev_states[proof_name]['value'],
                revoc_reg_def_json,
                revoc_reg_delta_json,
                timestamp,
                cred_rev_id[0 if proof_name == 'Alex' else 3]  # Alex, then Olga
            )

            # Prover creates revocation state from scratch
            # from: 0 or time of credential issuance
            # to: to
            # Prover Gets RevocationRegistryDelta from Ledger
            get_revoc_reg_delta_request_2 = await ledger.build_get_revoc_reg_delta_request(
                prover_did,
                rev_reg_def_id,
                0,
                to
            )

            get_revoc_reg_delta_response_2 = await ensure_previous_request_applied(
                pool_handle,
                get_revoc_reg_delta_request_2,
                lambda response: response['result']['seqNo'] is not None
            )
            (rev_reg_id, revoc_reg_delta_json_2, timestamp_2) = await ledger.parse_get_revoc_reg_delta_response(
                get_revoc_reg_delta_response_2
            )

            # Prover creates Revocation State
            rev_state_from_0_to_now_json = await anoncreds.create_revocation_state(
                blob_storage_reader_cfg_handle,
                revoc_reg_def_json,
                revoc_reg_delta_json_2,
                timestamp_2,
                cred_rev_id[0 if proof_name == 'Alex' else 3]  # Alex, then Olga
            )

            rev_states[proof_name] = {
                'timestamp': timestamp,
                'value': rev_state_from_timestamp_to_now_json # rev_state_from_0_to_now_json must also work
            }

            revoc_states_json = json.dumps({rev_reg_id: {timestamp: json.loads(rev_states[proof_name]['value'])}})

            # Prover Creates Proof for Proof Request
            requested_credentials_json = json.dumps(
                {
                    'self_attested_attributes': {},
                    'requested_attributes': {
                        'attr1_referent': {
                            'cred_id': cred_info[proof_name]['referent'],
                            'revealed': True,
                            'timestamp': timestamp
                        }
                    },
                    'requested_predicates': {
                        'pred1_referent': {
                            'cred_id': cred_info[proof_name]['referent'],
                            'timestamp': timestamp
                        }
                    }
                }
            )

            proof_json = await anoncreds.prover_create_proof(
                prover_wallet_handle,
                proof_req_json[proof_name],
                requested_credentials_json,
                master_secret_id,
                schemas_json,
                credential_defs_json,
                revoc_states_json
            )

            proof = json.loads(proof_json)
            identifier = proof['identifiers'][0]

            # Verifier Gets RevocationRegistry from Ledger
            get_revoc_reg_request = await ledger.build_get_revoc_reg_request(
                verifier_did,
                identifier['rev_reg_id'],
                identifier['timestamp']
            )
            get_revoc_reg_response = await ensure_previous_request_applied(
                pool_handle,
                get_revoc_reg_request,
                lambda response: response['result']['seqNo'] is not None
            )
            (rev_reg_id, rev_reg_json, timestamp) = await ledger.parse_get_revoc_reg_response(get_revoc_reg_response)

            revoc_regs_json = json.dumps({rev_reg_id: {timestamp: json.loads(rev_reg_json)}})
            print(Ink.RED('>> >> rev regs: {}'.format(ppjson(revoc_regs_json))))

            proof_req_touchup = json.loads(proof_req_json[proof_name])
            proof_req_touchup['non_revoked']['to'] = int(time.time())
            proof_req_json[proof_name] = json.dumps(proof_req_touchup)
            print(
                '.. .. {}: Proof req {}, proof {}'.format(
                    proof_name,
                    ppjson(proof_req_json[proof_name]),
                    ppjson(proof_json, 1024)
                )
            )
            verified = await anoncreds.verifier_verify_proof(
                proof_req_json[proof_name],
                proof_json,
                schemas_json,
                credential_defs_json,
                revoc_ref_defs_json,
                revoc_regs_json
            )
            assert proof_req_expected_result[proof_name] == verified
            print(
                Ink.GREEN(
                    '.. .. Proof for {} after {} revocation verifies as: {}'.format(
                        proof_name,
                        revo_name,
                        verified
                    )
                )
            )

    # Close and Delete Prover Wallet
    await wallet.close_wallet(prover_wallet_handle)
    await wallet.delete_wallet(prover_wallet_config, credentials)
