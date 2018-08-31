import json
from vcx.error import VcxError
from vcx.api import vcx_init, schema
from vcx.api.issuer_credential import IssuerCredential
from vcx.state import State
from vcx.api import connection
import asyncio

SCHEMA_SEQ_NUMBER = 22
SCHEMAS = {}
CREDENTIALS = {}

SCHEMA_22_SERIALIZED = "{'handle': 1718697361, 'name': '', 'sequence_num': 22, 'data': {'seqNo': 22, 'txnTime': 1516383536, 'identifier': '2hoqvcwupRTUNkXn6ArYzs', 'data': {'name': 'Home Address', 'version': '0.1', 'attr_names': ['address1', 'address2', 'city', 'state', 'zip']}, 'type': '101'}, 'source_id': 'test1'}"
SCHEMA_22_JSON = '{"sequence_num": 22, "handle": 3674279773, "data": {"type": "101", "data": {"version": "0.1", "attr_names": ["address1", "address2", "city", "state", "zip"], "name": "Home Address"}, "txnTime": 1516383536, "seqNo": 22, "identifier": "2hoqvcwupRTUNkXn6ArYzs"}, "name": "", "source_id": "test1"}'
FRANKS_DATA = {
    "address1": "123 Hobo Lane",
    "address2": "#44",
    "city": "Lexington",
    "state": "VA",
    "zip": "04142"
    }
# assumes a provisioned wallet
async def init():
    config = '/home/mark/Documents/cxsconfig.json'
    v = await vcx_init.vcx_init(config)
    return v


connection1 = None

async def create_and_connect():
    c1 = await connection.Connection.create('Demo1')
    state = await c1.connect('2053154785')

    print('\n %s \n' % str(json.dumps(await c1.invite_details(True))))
    while state != State.Accepted:
        print("waiting for connection to be accepted")
        await c1.update_state()
        state = await c1.get_state()
        print("state: %s"% state)
        await asyncio.sleep(5)
    print(state)
    return c1


async def schema_lookup(schema_seq_number):
    try:
        s1 = await schema.Schema.lookup('test1', schema_seq_number)
        return s1
    except VcxError as e:
        print("error: " + str(e))


def run_task(task, loop):
    tasks = [loop.create_task(task())]
    wait_tasks = asyncio.wait(tasks)
    loop.run_until_complete(wait_tasks)


async def create_issuer_credential(schema_seq_number, schema_attr, credential_name, source_id):
    return await IssuerCredential.create(source_id, schema_attr, schema_seq_number, credential_name)


async def send_credential_offer(issuer_credential, connection):
    await issuer_credential.send_offer(connection)


def main():
    loop = asyncio.get_event_loop()
    run_task(init, loop)
    tasks = schema_lookup(SCHEMA_SEQ_NUMBER)
    SCHEMAS[SCHEMA_SEQ_NUMBER] = loop.run_until_complete(asyncio.gather(tasks))[0]
    tasks = SCHEMAS[SCHEMA_SEQ_NUMBER].serialize()
    results = loop.run_until_complete(asyncio.gather(tasks))
    if len(results) > 0:
        schema_22 = json.dumps(results[0])
        print(schema_22)
        schema_22 = json.loads(schema_22)
        print(schema_22['handle'])
        task = create_issuer_credential(SCHEMA_SEQ_NUMBER, FRANKS_DATA, 'Generic Credential Name', '1')
        issuer_credential = loop.run_until_complete(asyncio.gather(task))
        print(issuer_credential)
        task = create_and_connect()
        results = loop.run_until_complete(asyncio.gather(task))
        if len(results) > 0:
            connection = results[0]
            print(connection)



        # task = send_credential_offer(issuer_credential, )
    loop.close()


def print_schema():
    parsed = json.loads(SCHEMA_22_JSON)
    print(json.dumps(parsed, indent=4, sort_keys=True))


if __name__ == '__main__':
    main()
    # print_schema()
