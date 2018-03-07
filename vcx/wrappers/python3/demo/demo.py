from demo.utils.create_vcx import update_json_values, config_dev
from demo.utils.random_name import get_random_name
from demo.vcxdemo import Vcxdemo
from demo.enterprise_settings import settings

from vcx.error import VcxError
from vcx.state import State
from vcx.api.connection import Connection
from vcx.api.issuer_claim import IssuerClaim
from vcx.api.schema import Schema
from vcx.api.proof import Proof

import qrcode
import json
import os
import time
import sys

SOURCE_ID = 'Philip J Fry'
DETAILS= [{
    "name": 'Phillip J Fry',
    "account": '435111368991'
},
    {
        "address1": "42 E 57th St.",
        "address2": "#44",
        "city": "New New York",
        "state": "NY",
        "zip": "12345"

    }]

QR_DATA= '{"sm": "message created", "t": "there", "s": {"v": "E2u2xA7RqRgGZ5aSyaJMGr1yEUfKUoyPot9qtnWJUXPC", "n": "DoomTown", "dp": {"k": "DVKwoWZ8PtYqGqSjEcExxggLc46iM91jW31qvgXABq9h", "s": "TW8FLdouR5KKb/35wNvSsi5pzcb59ycDzZCIftMSX2Tlmx2IFy1zmKopKj6L3EUfHaX9F1gQRggD7OEnkVFeDg==", "d": "PuxVjQ9imGLnVwrCbYukjn"}, "d": "Qut5k3WJVeDkvLx2Z5XHDe", "l": "https://robohash.org/469b25d"}, "id": "nzvmmwn", "sc": "MS-101", "sa": {"v": "4hmBc54YanNhQHTD66u6XDp1NSgQm1BacPFbE7b5gtat", "d": "7o2xT9Qtp83cJUJMUBTF3M", "e": "52.38.32.107:80/agency/msg"}}'
VCXCONFIG_PATH = 'utils/vcxconfig.json'
SCHEMA_SEQ_NUMBER = 22
CLAIM_NAME = 'Planet Express Club Member'
SCHEMA_NAME = 'Club Membership'


def write_json_to_file(data, filename, sort_keys=False):
    try:
        with open(filename, 'w') as out_file:
            json.dump(data, out_file, indent=4, sort_keys=sort_keys)
    except IOError as e:
        print('error writing to %s: %s' % (filename, e))
        sys.exit(1)


def init_vcxdemo(path):

    try:
        Vcxdemo.init(path)
    except VcxError as e:
        print('Error Initializing Vcx: ' + str(e))
        assert False


def util_create_schema(source_id, schema_name):
    attr_names = ['name', 'account']
    version = '1.0'
    schema_skeleton = {"name": schema_name,
                       "version": version,
                       "attr_names": attr_names,
                       }
    size = len(Vcxdemo.schemas)
    Vcxdemo.create_schema(source_id, schema_name, schema_skeleton)
    assert len(Vcxdemo.schemas) == size + 1


# THE demo
def test_demo():
    random_enterprise_name = get_random_name()
    genesis_path = settings['genesis_path']
    enterprise_config = {
        'institution_name': random_enterprise_name,
        'institution_logo_url': 'https://robohash.org/' + random_enterprise_name,
        'genesis_path': genesis_path,
        'wallet_name': settings['wallet_name'],
        'wallet_key': settings['wallet_key']
    }
    vcx_config_json = update_json_values(enterprise_config, config_dev)
    assert vcx_config_json['institution_name'] == random_enterprise_name
    write_json_to_file(vcx_config_json, 'utils/vcxconfig.json', sort_keys=True)

    schema_name = 'Club Membership'
    init_vcxdemo(VCXCONFIG_PATH)
    claim_name = 'Club Membership'
    schema_source_id = random_enterprise_name
    claim_def_source_id = random_enterprise_name
    connection_source_id = random_enterprise_name
    assert len(Vcxdemo.schemas) == 0
    util_create_schema(schema_source_id, schema_name)
    write_json_to_file(Vcxdemo.serialize_schema(0), 'schema.dat')

    # Write Connection to file
    assert len(Vcxdemo.schemas) == 1
    s0 = Vcxdemo.get_schema_attr_list(0)

    # Create Claim Def on Ledger (and wallet)
    schema_sequence_number = Vcxdemo.get_schema_sequence_number(0)
    Vcxdemo.create_claim_def(claim_def_source_id, claim_name, schema_sequence_number)
    assert len(Vcxdemo.claim_defs) > 0
    assert Vcxdemo.claim_defs[claim_name]

    customer1 = Vcxdemo(connection_source_id)

    # Connection
    assert customer1.state['connection'] == State.Undefined
    customer1.connect()
    assert type(customer1.connection) == Connection
    assert customer1.state['connection'] == State.Accepted
    # Write Connection to file
    write_json_to_file(customer1.serialize_connection(), 'connection.dat')

    # Claim
    customer1.create_claim(Vcxdemo.get_schema_sequence_number(0), DETAILS[0], claim_name)
    assert type(customer1.claim) == IssuerClaim
    customer1.update_claim_state()
    assert customer1.state['claim'] == State.Initialized
    # Write Claim to file
    write_json_to_file(customer1.serialize_claim(), 'claim.dat')
    customer1.update_claim_state()
    assert customer1.state['claim'] == State.Initialized
    customer1.issue_claim_offer()
    customer1.update_claim_state()
    assert customer1.state['claim'] == State.OfferSent
    customer1.wait_for_claim_state(State.RequestReceived)
    print("About to send claim offer...")
    # time.sleep(5)
    customer1.send_issuer_claim()
    print("Waiting for claim state to become accepted")
    customer1.wait_for_claim_state(State.Accepted)
    print("Claim State became accepted")
    time.sleep(5)
    # Proof
    proof_id = '222'
    proof_attr = Vcxdemo.format_proof_attrs(Vcxdemo.did,
                                            Vcxdemo.get_schema_sequence_number(0),
                                            Vcxdemo.get_schema_attr_list(0)['attr_names'])
    source_id = 'Club Membership'

    request_proof(customer1, source_id, proof_attr, proof_id)


def request_proof(connection, source_id, proof_attr, proof_id):
    connection.create_proof_request(proof_id, source_id, proof_attr)
    assert isinstance(connection.connection, Connection)
    connection.request_proof(proof_id)
    connection.wait_for_proof_state(proof_id, State.OfferSent)
    assert connection.get_proof_state(proof_id) == State.OfferSent
    connection.wait_for_proof_state(proof_id, State.Accepted)
    assert connection.get_proof_state(proof_id) == State.Accepted
    proof = connection.retrieve_proof(proof_id)
    return proof


# demo
def test_vcx_deserialize_connection_fulfill_claim():
    try:
        Vcxdemo.init(VCXCONFIG_PATH)
    except VcxError as e:
        assert False

    frank = Vcxdemo(SOURCE_ID, details=DETAILS)
    frank.deserialize_connection('connection.dat')
    assert type(frank.connection) == Connection
    # Claim
    frank.create_claim(SCHEMA_SEQ_NUMBER, DETAILS[1], 'Planet Express Info')
    assert type(frank.claim) == IssuerClaim
    frank.update_claim_state()
    assert frank.state['claim'] == State.Initialized
    frank.issue_claim_offer()
    assert frank.state['claim'] == State.OfferSent
    frank.wait_for_claim_state(State.RequestReceived)
    assert frank.state['claim'] == State.RequestReceived
    # write claim out to claim2.dat
    try:
        with open('claim.dat', 'w') as out_file:
            json.dump(frank.serialize_claim(), out_file)
    except IOError as e:
        print('error writing to claim.dat: %s' % e)


# demo
def test_deserialize_accepted_claim_and_issue_claim():
    init_vcxdemo()
    frank = Vcxdemo(SOURCE_ID, details=DETAILS)
    frank.deserialize_connection('connection.dat')
    frank.deserialize_claim('claim2.dat')
    assert type(frank.claim) == IssuerClaim
    frank.send_issuer_claim()
    assert frank.state['claim'] == State.Accepted


# demo
def test_insert_claim_def_into_wallet():
    init_vcxdemo()
    schema_name = 'Account Ledger'
    attr_names = ['name', 'account']
    source_id = 'Planet Express'
    version = '1.0'
    schema_skeleton = {"name": source_id,
                       "version": version,
                       "attr_names": attr_names,
                       }
    Vcxdemo.create_schema(source_id, schema_name, schema_skeleton)
    assert len(Vcxdemo.schemas) > 0
    Vcxdemo.create_schema(source_id, schema_name, schema_skeleton)
    assert len(Vcxdemo.schemas) == 2
    write_json_to_file(Vcxdemo.serialize_schema(0), 'schema.dat')

    # Claim Def
    name = 'Customer'
    schema_number = Vcxdemo.get_schema_sequence_number(0)
    Vcxdemo.create_claim_def(source_id, name, schema_number)
    assert len(Vcxdemo.claim_defs) > 0
    assert Vcxdemo.claim_defs[name]


# demo
def test_connection_request_qr_code():
    img = qrcode.make(QR_DATA)
    img.save('./qrcode1.png')


# demo
def test_claim_data():
    c = None
    with open('claim.dat') as in_file:
        c = json.load(in_file)
    assert c['issuer_did'] == '2hoqvcwupRTUNkXn6ArYzs'


# unit testable
def test_format_proof_attr():
    did = ENTERPRISE_DID
    schema_seq_number = 555
    attr_list = ['name', 'account']
    id = '12345'
    Vcxdemo.set_did(ENTERPRISE_DID)
    assert Vcxdemo.get_did() == ENTERPRISE_DID
    leela = Vcxdemo('leela')
    formatted_attr = Vcxdemo._create_attr(attr_list[0], did, schema_seq_number)
    assert formatted_attr['schema_seq_no'] == schema_seq_number
    assert formatted_attr['issuer_did'] == did
    assert formatted_attr['name'] == attr_list[0]
    proof_attr = Vcxdemo.format_proof_attrs(did, schema_seq_number, attr_list)
    assert len(proof_attr) == len(attr_list)
    assert proof_attr[0]['schema_seq_no'] == schema_seq_number
    leela.create_proof_request(id, 'Club Membership Proof', proof_attr)
    assert len(leela.proof_requests) > 0
    assert isinstance(leela.get_proof_request(id), Proof)


# unit testable
def test_schema():
    expected_attrs = {
        "name": "Planet Express",
        "version": "1.0",
        "attr_names": [
            "name",
            "account"
        ]
    }
    Vcxdemo.init(VCXCONFIG_PATH)
    filename = 'schema.dat'
    data = None
    try:
        with open(filename) as in_file:
            data = json.load(in_file)
    except IOError as e:
        print('error reading/opening %s: %s', (filename, e))

    # deserialize from schema.dat file
    Vcxdemo.deserialize_schema(filename)
    assert len(Vcxdemo.schemas) == 1
    assert isinstance(Vcxdemo.schemas[0], Schema)
    assert Vcxdemo.get_schema_sequence_number(0) == data['sequence_num']
    s0 = Vcxdemo.get_schema_attr_list(0)
    s1 = data['data']['data']
    assert s0 == s1
    assert len(Vcxdemo.schemas) > 0
    assert isinstance(Vcxdemo.get_schema(0), Schema)
    util_create_schema('Third Schema')
    assert len(Vcxdemo.schemas) == 2
    s_created = Vcxdemo.get_schema_attr_list(1)
    Vcxdemo.deserialize_schema('schema.dat')
    assert len(Vcxdemo.schemas) == 3
    s_deserialized = Vcxdemo.get_schema_attr_list(2)
    assert s_deserialized == s0
    assert s_created == s0


# demo
def test_request_proof():
    Vcxdemo.init(VCXCONFIG_PATH)
    customer1 = Vcxdemo('Fry')
    customer1.deserialize_connection('connection.dat')
    assert isinstance(customer1.connection, Connection)
    customer1.deserialize_claim('claim.dat')
    Vcxdemo.deserialize_schema('schema.dat')

    proof_id = '222'
    proof_attr = Vcxdemo.format_proof_attrs(Vcxdemo.did,
                                            Vcxdemo.get_schema_sequence_number(0),
                                            Vcxdemo.get_schema_attr_list(0)['attr_names'])
    customer1.create_proof_request(proof_id,'Club Membership', proof_attr)
    customer1.wait_for_proof_state(proof_id, State.Initialized)
    assert customer1.get_proof_state(proof_id) == State.Initialized
    customer1.request_proof(proof_id)
    customer1.wait_for_proof_state(proof_id, State.OfferSent)
    assert customer1.get_proof_state(proof_id) == State.OfferSent
    customer1.wait_for_proof_state(proof_id, State.Accepted)
    assert customer1.get_proof_state(proof_id) == State.Accepted
    customer1.retrieve_proof(proof_id)

# unit testable
def test_utils_create_config():
    filename = 'vcxconfig.json'
    dir = 'utils'
    image_hash= '12345'
    random_name = get_random_name()
    config = {
        'institution_logo_url': 'https://robohash.org/' + image_hash,
        'institution_name': random_name
    }
    create_vcx_config_json(config)
    assert filename in os.listdir(dir)
    with open(os.path.join(dir, filename), 'r') as in_file:
        config = json.load(in_file)
        assert config['institution_logo_url'] == 'https://robohash.org/' + image_hash
        assert config['institution_name'] == random_name
    print(get_random_name())


# demo
def test_lookup():
    schema_filename = 'schema.dat'
    schema = None
    with open(schema_filename) as in_file:
        schema = json.load(in_file)

    Vcxdemo.init(VCXCONFIG_PATH)
    customer1 = Vcxdemo('Fry')
    customer1.deserialize_connection('connection.dat')
    assert isinstance(customer1.connection, Connection)
    customer1.deserialize_claim('claim.dat')
    Vcxdemo.deserialize_schema(schema_filename)
    lookup_schema = Vcxdemo.lookup_schema('FindSchema', Vcxdemo.get_schema_sequence_number(0))
    Vcxdemo.schemas.append(lookup_schema)
    assert len(Vcxdemo.schemas) == 2
    assert schema['sequence_num'] == Vcxdemo.get_schema_sequence_number(1)


if __name__=='__main__':
    demo()
