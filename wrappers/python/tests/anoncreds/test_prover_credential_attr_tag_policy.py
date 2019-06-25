from indy import anoncreds
from indy.error import ErrorCode, IndyError

import json
import pytest


async def _check_catpol(wallet_handle, cred_def_json, cred_def_id, cred_id, cred_value, offer_json, cred_req,
                        cred_req_metadata, taggables, retroactive, query_attrs, expect_cred_ids, expect_policy):
    # Set policy
    await anoncreds.prover_set_credential_attr_tag_policy(wallet_handle, cred_def_id, taggables, retroactive)

    # Write credential
    (cred, _, _) = await anoncreds.issuer_create_credential(wallet_handle, offer_json, cred_req,
                                                    json.dumps(cred_value), None, None)
    await anoncreds.prover_store_credential(wallet_handle, cred_id, cred_req_metadata, cred, cred_def_json, None)

    # Search on all tags
    query_json = json.dumps({
        **{'attr::{}::marker'.format(attr): '1' for attr in query_attrs},
        **{'attr::{}::value'.format(attr): cred_value[attr]['raw'] for attr in query_attrs},
    })
    (handle, count) = await anoncreds.prover_search_credentials(wallet_handle, query_json)

    found = json.loads(
        await anoncreds.prover_fetch_credentials(handle, count))

    assert {cred['referent'] for cred in found} == (expect_cred_ids or set())
    await anoncreds.prover_close_credentials_search(handle)

    # Get and check current policy
    catpol = json.loads(await anoncreds.prover_get_credential_attr_tag_policy(wallet_handle, cred_def_id))
    if expect_policy is None:
        assert catpol is None
    else:
        assert set(catpol) == expect_policy

async def _check_query(wallet_handle, cred_value, attr, expect_cred_ids):
    query_json = json.dumps({
        'attr::{}::marker'.format(attr): '1',
        'attr::{}::value'.format(attr): cred_value[attr]['raw']
    })
    (handle, count) = await anoncreds.prover_search_credentials(wallet_handle, query_json)

    found = json.loads(
        await anoncreds.prover_fetch_credentials(handle, count))

    assert {cred['referent'] for cred in found} == (expect_cred_ids or set())
    await anoncreds.prover_close_credentials_search(handle)


# noinspection PyUnusedLocal
@pytest.mark.asyncio
async def test_prover_credential_attr_tag_policy(wallet_handle, prepopulated_wallet, issuer_1_gvt_cred_def_id):
    cred_values = {
        i: {
            'sex': {
                'raw': ('male', 'female')[i % 2],
                'encoded': ('123456789012', '135791357902')[i % 2]
            },
            'name': {
                'raw': ('Wayne', 'Hailey', 'Sidney', 'Cammi', 'Connor')[i],
                'encoded': ('987654321098', '876543210987', '765432109876', '654321098765', '543210987654')[i]
            },
            'height': {
                'raw': ('180', '160', '183', '161', '192')[i],
                'encoded': ('180', '160', '183', '161', '192')[i]
            },
            'age': {
                'raw': str(60 + i),
                'encoded': str(60 + i)
            }
        } for i in range(5)
    }

    (cred_def_json, offer_json, cred_req, cred_req_metadata) = prepopulated_wallet[0:4]

    # SET POLICY NON-RETROACTIVELY when wallet has no credentials of interest

    # Null policy (default, all attrs)
    await _check_catpol(wallet_handle=wallet_handle,
                        cred_def_json=cred_def_json,
                        cred_def_id=issuer_1_gvt_cred_def_id,
                        cred_id='cred-0.0',
                        cred_value=cred_values[0],
                        offer_json=offer_json,
                        cred_req=cred_req,
                        cred_req_metadata=cred_req_metadata,
                        taggables=None,
                        retroactive=False,
                        query_attrs=[attr for attr in cred_values[0]],
                        expect_cred_ids={'cred-0.0'},
                        expect_policy=None)

    # No-attrs policy
    await _check_catpol(wallet_handle=wallet_handle,
                        cred_def_json=cred_def_json,
                        cred_def_id=issuer_1_gvt_cred_def_id,
                        cred_id='cred-0.1',
                        cred_value=cred_values[1],
                        offer_json=offer_json,
                        cred_req=cred_req,
                        cred_req_metadata=cred_req_metadata,
                        taggables=json.dumps([]),
                        retroactive=False,
                        query_attrs=['name'],
                        expect_cred_ids=None,
                        expect_policy=set())

    # One-attr policy
    await _check_catpol(wallet_handle=wallet_handle,
                        cred_def_json=cred_def_json,
                        cred_def_id=issuer_1_gvt_cred_def_id,
                        cred_id='cred-0.2',
                        cred_value=cred_values[2],
                        offer_json=offer_json,
                        cred_req=cred_req,
                        cred_req_metadata=cred_req_metadata,
                        taggables=json.dumps(['name']),
                        retroactive=False,
                        query_attrs=['name'],
                        expect_cred_ids={'cred-0.2'},
                        expect_policy={'name'})
    await _check_query(wallet_handle=wallet_handle,  # also ensure wallet does not tag untaggable attrs
                       cred_value=cred_values[2],
                       attr='age',
                       expect_cred_ids=None)

    # All-but-one-attr policy
    await _check_catpol(wallet_handle=wallet_handle,
                        cred_def_json=cred_def_json,
                        cred_def_id=issuer_1_gvt_cred_def_id,
                        cred_id='cred-0.3',
                        cred_value=cred_values[3],
                        offer_json=offer_json,
                        cred_req=cred_req,
                        cred_req_metadata=cred_req_metadata,
                        taggables=json.dumps(['sex', 'height', 'age']),
                        retroactive=False,
                        query_attrs=['sex', 'height', 'age'],
                        expect_cred_ids={'cred-0.3'},
                        expect_policy={'sex', 'height', 'age'})
    await _check_query(wallet_handle=wallet_handle,  # also ensure wallet does not tag untaggable attrs
                       cred_value=cred_values[3],
                       attr='name',
                       expect_cred_ids=None)

    # Explicit all-attrs policy
    await _check_catpol(wallet_handle=wallet_handle,
                        cred_def_json=cred_def_json,
                        cred_def_id=issuer_1_gvt_cred_def_id,
                        cred_id='cred-0.4',
                        cred_value=cred_values[4],
                        offer_json=offer_json,
                        cred_req=cred_req,
                        cred_req_metadata=cred_req_metadata,
                        taggables=json.dumps([attr for attr in cred_values[4]]),
                        retroactive=False,
                        query_attrs=[attr for attr in cred_values[4]],
                        expect_cred_ids={'cred-0.4'},
                        expect_policy={attr for attr in cred_values[4]})

    # SET POLICY RETROACTIVELY

    # Null policy (default, all attrs)
    await _check_catpol(wallet_handle=wallet_handle,
                        cred_def_json=cred_def_json,
                        cred_def_id=issuer_1_gvt_cred_def_id,
                        cred_id='cred-1.0',
                        cred_value=cred_values[0],
                        offer_json=offer_json,
                        cred_req=cred_req,
                        cred_req_metadata=cred_req_metadata,
                        taggables=None,
                        retroactive=True,
                        query_attrs=[attr for attr in cred_values[0]],
                        expect_cred_ids={'cred-0.0', 'cred-1.0'},
                        expect_policy=None)

    # No-attrs policy
    await _check_catpol(wallet_handle=wallet_handle,
                        cred_def_json=cred_def_json,
                        cred_def_id=issuer_1_gvt_cred_def_id,
                        cred_id='cred-1.1',
                        cred_value=cred_values[1],
                        offer_json=offer_json,
                        cred_req=cred_req,
                        cred_req_metadata=cred_req_metadata,
                        taggables=json.dumps([]),
                        retroactive=True,
                        query_attrs=['name'],
                        expect_cred_ids=None,
                        expect_policy=set())

    # One-attr policy
    await _check_catpol(wallet_handle=wallet_handle,
                        cred_def_json=cred_def_json,
                        cred_def_id=issuer_1_gvt_cred_def_id,
                        cred_id='cred-1.2',
                        cred_value=cred_values[2],
                        offer_json=offer_json,
                        cred_req=cred_req,
                        cred_req_metadata=cred_req_metadata,
                        taggables=json.dumps(['name']),
                        retroactive=True,
                        query_attrs=['name'],
                        expect_cred_ids={'cred-0.2', 'cred-1.2'},
                        expect_policy={'name'})
    await _check_query(wallet_handle=wallet_handle,  # also ensure wallet does not tag untaggable attrs
                       cred_value=cred_values[2],
                       attr='age',
                       expect_cred_ids=None)

    # All-but-one-attr policy
    await _check_catpol(wallet_handle=wallet_handle,
                        cred_def_json=cred_def_json,
                        cred_def_id=issuer_1_gvt_cred_def_id,
                        cred_id='cred-1.3',
                        cred_value=cred_values[3],
                        offer_json=offer_json,
                        cred_req=cred_req,
                        cred_req_metadata=cred_req_metadata,
                        taggables=json.dumps(['sex', 'height', 'age']),
                        retroactive=True,
                        query_attrs=['sex', 'height', 'age'],
                        expect_cred_ids={'cred-0.3', 'cred-1.3'},
                        expect_policy={'sex', 'height', 'age'})
    await _check_query(wallet_handle=wallet_handle,  # also ensure wallet does not tag untaggable attrs
                       cred_value=cred_values[3],
                       attr='name',
                       expect_cred_ids=None)

    # Explicit all-attrs policy
    await _check_catpol(wallet_handle=wallet_handle,
                        cred_def_json=cred_def_json,
                        cred_def_id=issuer_1_gvt_cred_def_id,
                        cred_id='cred-1.4',
                        cred_value=cred_values[4],
                        offer_json=offer_json,
                        cred_req=cred_req,
                        cred_req_metadata=cred_req_metadata,
                        taggables=json.dumps([attr for attr in cred_values[4]]),
                        retroactive=True,
                        query_attrs=[attr for attr in cred_values[4]],
                        expect_cred_ids={'cred-0.4', 'cred-1.4'},
                        expect_policy={attr for attr in cred_values[4]})

    # SET POLICY NON-RETROACTIVELY when wallet has some credentials of interest

    # Null policy (default, all attrs)
    await _check_catpol(wallet_handle=wallet_handle,
                        cred_def_json=cred_def_json,
                        cred_def_id=issuer_1_gvt_cred_def_id,
                        cred_id='cred-2.0',
                        cred_value=cred_values[0],
                        offer_json=offer_json,
                        cred_req=cred_req,
                        cred_req_metadata=cred_req_metadata,
                        taggables=None,
                        retroactive=False,
                        query_attrs=[attr for attr in cred_values[0]],
                        expect_cred_ids={'cred-0.0', 'cred-1.0', 'cred-2.0'},
                        expect_policy=None)

    # No-attrs policy
    await _check_catpol(wallet_handle=wallet_handle,
                        cred_def_json=cred_def_json,
                        cred_def_id=issuer_1_gvt_cred_def_id,
                        cred_id='cred-2.1',
                        cred_value=cred_values[1],
                        offer_json=offer_json,
                        cred_req=cred_req,
                        cred_req_metadata=cred_req_metadata,
                        taggables=json.dumps([]),
                        retroactive=False,
                        query_attrs=['name'],
                        expect_cred_ids={'cred-0.1', 'cred-1.1'},
                        expect_policy=set())

    # One-attr policy
    await _check_catpol(wallet_handle=wallet_handle,
                        cred_def_json=cred_def_json,
                        cred_def_id=issuer_1_gvt_cred_def_id,
                        cred_id='cred-2.2',
                        cred_value=cred_values[2],
                        offer_json=offer_json,
                        cred_req=cred_req,
                        cred_req_metadata=cred_req_metadata,
                        taggables=json.dumps(['name']),
                        retroactive=False,
                        query_attrs=['name'],
                        expect_cred_ids={'cred-0.2', 'cred-1.2', 'cred-2.2'},
                        expect_policy={'name'})
    await _check_query(wallet_handle=wallet_handle,  # also ensure wallet does not tag untaggable attrs
                       cred_value=cred_values[2],
                       attr='age',
                       expect_cred_ids={'cred-0.2', 'cred-1.2'})

    # All-but-one-attr policy
    await _check_catpol(wallet_handle=wallet_handle,
                        cred_def_json=cred_def_json,
                        cred_def_id=issuer_1_gvt_cred_def_id,
                        cred_id='cred-2.3',
                        cred_value=cred_values[3],
                        offer_json=offer_json,
                        cred_req=cred_req,
                        cred_req_metadata=cred_req_metadata,
                        taggables=json.dumps(['sex', 'height', 'age']),
                        retroactive=False,
                        query_attrs=['sex', 'height', 'age'],
                        expect_cred_ids={'cred-0.3', 'cred-1.3', 'cred-2.3'},
                        expect_policy={'sex', 'height', 'age'})
    await _check_query(wallet_handle=wallet_handle,  # also ensure wallet does not tag untaggable attrs
                       cred_value=cred_values[3],
                       attr='name',
                       expect_cred_ids={'cred-0.3', 'cred-1.3'})

    # Explicit all-attrs policy
    await _check_catpol(wallet_handle=wallet_handle,
                        cred_def_json=cred_def_json,
                        cred_def_id=issuer_1_gvt_cred_def_id,
                        cred_id='cred-2.4',
                        cred_value=cred_values[4],
                        offer_json=offer_json,
                        cred_req=cred_req,
                        cred_req_metadata=cred_req_metadata,
                        taggables=json.dumps([attr for attr in cred_values[4]]),
                        retroactive=False,
                        query_attrs=[attr for attr in cred_values[4]],
                        expect_cred_ids={'cred-0.4', 'cred-1.4', 'cred-2.4'},
                        expect_policy={attr for attr in cred_values[4]})

    # RESTORE wallet state: delete credentials created in this test
    for i in range(3):
        for j in range(5):
            await anoncreds.prover_delete_credential(wallet_handle, 'cred-{}.{}'.format(i, j))

    credentials = json.loads(await anoncreds.prover_get_credentials(wallet_handle, "{}"))
    assert len(credentials) == 3


# noinspection PyUnusedLocal
@pytest.mark.asyncio
async def test_prover_credential_attr_tag_policy_works_for_invalid_wallet(wallet_handle,
                                                                          prepopulated_wallet,
                                                                          issuer_1_gvt_cred_def_id):
    invalid_wallet_handle = wallet_handle + 100

    with pytest.raises(IndyError) as e:
        await anoncreds.prover_set_credential_attr_tag_policy(invalid_wallet_handle,
                                                              issuer_1_gvt_cred_def_id, None, False)

    assert ErrorCode.WalletInvalidHandle == e.value.error_code

    with pytest.raises(IndyError) as e:
        await anoncreds.prover_get_credential_attr_tag_policy(invalid_wallet_handle, issuer_1_gvt_cred_def_id)

    assert ErrorCode.WalletInvalidHandle == e.value.error_code


# noinspection PyUnusedLocal
@pytest.mark.asyncio
async def test_prover_credential_attr_tag_policy_works_for_redundant_attr(wallet_handle,
                                                                          prepopulated_wallet,
                                                                          issuer_1_gvt_cred_def_id):

    # Set policy
    await anoncreds.prover_set_credential_attr_tag_policy(wallet_handle, issuer_1_gvt_cred_def_id,
                                                          json.dumps(['age', 'age']), False)

    # Get and check current policy
    catpol = json.loads(await anoncreds.prover_get_credential_attr_tag_policy(wallet_handle, issuer_1_gvt_cred_def_id))
    assert catpol == ['age']

    # Clear policy
    await anoncreds.prover_set_credential_attr_tag_policy(wallet_handle, issuer_1_gvt_cred_def_id, None, False)


# noinspection PyUnusedLocal
@pytest.mark.asyncio
async def test_prover_credential_attr_tag_policy_works_for_non_canonical_attr(wallet_handle,
                                                                              prepopulated_wallet,
                                                                              issuer_1_gvt_cred_def_id):
    cred_value = {
        'sex': {
            'raw': 'female',
            'encoded': '135791357902'
        },
        'name': {
            'raw': 'Eveliina',
            'encoded': '321098765432'
        },
        'height': {
            'raw': '162',
            'encoded': '162'
        },
        'age': {
            'raw': str(65),
            'encoded': str(65)
        }
    } 

    (cred_def_json, offer_json, cred_req, cred_req_metadata) = prepopulated_wallet[0:4]

    # One-attr policy, specified with non-canonical attr
    await _check_catpol(wallet_handle=wallet_handle,
                        cred_def_json=cred_def_json,
                        cred_def_id=issuer_1_gvt_cred_def_id,
                        cred_id='cred-eve',
                        cred_value=cred_value,
                        offer_json=offer_json,
                        cred_req=cred_req,
                        cred_req_metadata=cred_req_metadata,
                        taggables=json.dumps(['NAME']),
                        retroactive=False,
                        query_attrs=['name'],
                        expect_cred_ids={'cred-eve'},
                        expect_policy={'name'})

    # Restore wallet state: delete credentials created in this test
    await anoncreds.prover_delete_credential(wallet_handle, 'cred-eve')

    credentials = json.loads(await anoncreds.prover_get_credentials(wallet_handle, "{}"))
    assert len(credentials) == 3
