from vcx.api.connection import Connection
from demo.wait import wait_for_state
import json
import asyncio
from vcx.state import State
from vcx.api.vcx_init import vcx_init
from vcx.api.issuer_claim import IssuerClaim
from vcx.api.schema import Schema
from vcx.api.claim_def import ClaimDef
from vcx.api.proof import Proof
import qrcode

ENTERPRISE_DID = '2hoqvcwupRTUNkXn6ArYzs'


class Vcxdemo:

    proof_requests = {}
    schemas = []
    claim_defs = {}
    did = ENTERPRISE_DID

    def __init__(self, source_id, details=None, phone_number='8888675309'):
        self.source_id = source_id
        self.details = details
        self.state = {}
        self.state['connection'] = State.Undefined
        self.state['claim'] = State.Undefined
        self.loop = Vcxdemo.get_loop()
        self.connection = None
        self.claim = None
        self.phone_number = phone_number
        self.invite_details = None

    @classmethod
    def set_did(cls, did):
        cls.did = did

    @classmethod
    def get_did(cls):
        return cls.did

    @staticmethod
    def get_loop():
        if not hasattr(Vcxdemo.get_loop, "loop"):
            Vcxdemo.get_loop.loop = asyncio.get_event_loop()
        return Vcxdemo.get_loop.loop

    @staticmethod
    def init(path):
        results = Vcxdemo.get_loop().run_until_complete(asyncio.gather(vcx_init(path)))
        print(results)
        return 1

    def create_qr_code(self, dest):
        img = qrcode.make(str(json.dumps(self.invite_details)))
        img.save(dest)



    async def _wait_for_claim_state(self, target_state):
        self.state['claim'] = await self.claim.update_state()
        while self.state['claim'] != target_state:
            print('waiting for claim to be [%s]...\ncurrent %s' % (target_state, self.state['claim']))
            await asyncio.sleep(5)
            self.state['claim'] = await self.claim.update_state()
        print('Successful state change for claim to be [%s]...\ncurrent %s' % (target_state, self.state['claim']))

    async def create_and_connect(self):
        self.connection = await Connection.create(self.source_id)
        self.state['connection'] = await self.connection.get_state()
        await self.connection.connect(self.phone_number)
        self.invite_details = await self.connection.invite_details(True)
        print('\n %s \n' % str(json.dumps(self.invite_details)))
        self.create_qr_code('./qrcode1.png')
        self.state['connection'] = await self.wait_for_connection_state(State.Accepted)


    def connect(self):
        self.loop.run_until_complete(asyncio.gather(self.create_and_connect()))

    def create_claim(self, schema_seq_number, attr, claim_name):
        self.loop.run_until_complete(asyncio.gather(self._create_claim(schema_seq_number, attr, claim_name)))

    async def _create_claim(self, schema_seq_number, attr, claim_name):
        self.claim = await IssuerClaim.create(self.source_id,
                                              attr,
                                              schema_seq_number,
                                              claim_name)

    def request_proof(self, proof_id):
        proof = self.get_proof_request(proof_id)
        res = Vcxdemo.get_loop().run_until_complete(proof.request_proof(self.connection))

    async def _serialize_connection(self):
        return await self.connection.serialize()

    def serialize_connection(self):
        res = self.loop.run_until_complete(asyncio.gather(self._serialize_connection()))
        if len(res) > 0:
            return res[0]

    async def _serialize_claim(self):
        return await self.claim.serialize()

    def serialize_claim(self):
        res = self.loop.run_until_complete(asyncio.gather(self._serialize_claim()))
        if len(res) > 0:
            return res[0]

    async def _deserialize_connection(self, data):
        self.connection = await Connection.deserialize(data)

    async def _deserialize_claim(self, data):
        self.claim = await IssuerClaim.deserialize(data)

    def deserialize_connection(self, filename):
        try:
            with open(filename) as in_file:
                res = self.loop.run_until_complete(asyncio.gather(self._deserialize_connection(json.load(in_file))))
        except IOError as e:
            print("Error opening file %s: %s" % (filename, e))

    def deserialize_claim(self,filename):
        try:
            with open(filename) as in_file:
                res = self.loop.run_until_complete(asyncio.gather(self._deserialize_claim(json.load(in_file))))
        except IOError as e:
            print("Error opening file %s: %s" % (filename, e))

    async def _update_claim_state(self):
        self.state['claim'] = await self.claim.update_state()

    async def _update_proof_state(self, proof_id):
        await self.get_proof_request(proof_id).update_state()

    async def _get_proof_state(self, proof_id):
        return await self.get_proof_request(proof_id).get_state()

    def update_proof_state(self, proof_id):
        Vcxdemo.get_loop().run_until_complete(asyncio.gather(self._update_proof_state(proof_id)))

    def get_proof_state(self, proof_id):
        res = Vcxdemo.get_loop().run_until_complete(asyncio.gather(self._get_proof_state(proof_id)))
        return res[0]

    def update_claim_state(self):
        res = self.loop.run_until_complete(asyncio.gather(self._update_claim_state()))
        if len(res) > 0:
            return res[0]

    async def _send_offer(self):
        await self.claim.send_offer(self.connection)
        await self.claim.update_state()
        self.state['claim'] = await self.claim.get_state()

    def issue_claim_offer(self):
        res = self.loop.run_until_complete(asyncio.gather(self._send_offer()))
        if len(res) > 0:
            return res[0]

    async def _send_issuer_claim(self):
        await self.claim.send_claim(self.connection)
        await self.claim.update_state()

    def send_issuer_claim(self):
        res = self.loop.run_until_complete(asyncio.gather(self._send_issuer_claim()))
        if len(res) > 0:
            return res[0]

    @staticmethod
    def _do_async_method(fn, kwargs):
        res = Vcxdemo.get_loop().run_until_complete(asyncio.gather(fn(**kwargs)))
        if len(res) > 0:
            return res[0]

    @staticmethod
    async def _create_schema(source_id: str, name:str, attr:dict):
        return await Schema.create(source_id, name, attr)

    @classmethod
    def create_schema(cls, source_id: str, name: str, attr: dict):
        res = Vcxdemo.get_loop().run_until_complete(asyncio.gather(Vcxdemo._create_schema(source_id, name, attr)))
        print(res[0])
        cls.schemas.append(res[0])

    @classmethod
    def get_schema(cls, index):
        return cls.schemas[index]

    @staticmethod
    async def _serialize_schema(schema):
        return await schema.serialize()

    @classmethod
    def serialize_schema(cls, schema_number):
        res = Vcxdemo.get_loop().run_until_complete(asyncio.gather(
            cls._serialize_schema(cls.schemas[schema_number])))
        if len(res) > 0:
            return res[0]

    @classmethod
    def get_schema_sequence_number(cls, index):
        res = Vcxdemo.get_loop().run_until_complete(asyncio.gather(cls.schemas[index].get_sequence_number()))
        return res[0]

    @classmethod
    def deserialize_schema(cls, filename):
        try:
            with open(filename, 'r') as in_file:
                data = json.load(in_file)
                res = Vcxdemo.get_loop().run_until_complete(asyncio.gather(Schema.deserialize(data)))
                cls.schemas.append(res[0])
        except IOError as e:
            print('Error opening %s: %s', (filename, e))

    @classmethod
    def create_claim_def(cls, source_id, name, schema_number, revocation=False):
        cls.claim_defs[name] = Vcxdemo.get_loop().run_until_complete(ClaimDef.create(source_id, name, schema_number, revocation))

    @staticmethod
    def _create_attr(attr: str, did: str, schema_seq_number: int):
        return {'schema_seq_no': schema_seq_number, 'name': attr, 'issuer_did': did}

    @staticmethod
    def format_proof_attrs(did: str, schema_seq_number: int, attr_list: list):
        formatted_attrs = []
        for a in attr_list:
            formatted_attrs.append(Vcxdemo._create_attr(a, did, schema_seq_number))

        return formatted_attrs

    def create_proof_request(self, source_id, name, proof_attr):
        res = Vcxdemo.get_loop().run_until_complete(asyncio.gather(Proof.create(source_id, name, proof_attr)))
        if len(res) > 0:
            self.proof_requests[source_id] = res[0]

    def get_proof_request(self, source_id):
        return self.proof_requests[source_id]

    @classmethod
    def get_schema_attr_list(cls, index):
        return cls.schemas[index].attrs

    async def _wait_for_proof_state(self, proof_id, target_state):
        proof = self.get_proof_request(proof_id)
        state = await proof.get_state()
        while state != target_state:
            print('waiting for Proof Request %s to be [%s]...\ncurrent %s' % (proof_id, target_state, state))
            await asyncio.sleep(5)
            await proof.update_state()
            state = await proof.get_state()
        print('Successful state change for Proof Request %s to [%s]...\ncurrent %s' % (proof_id, target_state, state))

    async def wait_for_connection_state(self, target_state):
        await self.connection.update_state()
        state = await self.connection.get_state()
        while state != target_state:
            print('waiting for connection to be accepted...\ncurrent %s' % state)
            await asyncio.sleep(5)
            await self.connection.update_state()
            state = await self.connection.get_state()
        return state

    def wait_for_claim_state(self, target_state):
        self.loop.run_until_complete(asyncio.gather(self._wait_for_claim_state(target_state)))

    def wait_for_proof_state(self, proof_id, target_state):
        proof = self.get_proof_request(proof_id)
        # Vcxdemo.get_loop().run_until_complete(asyncio.gather(self._wait_for_proof_state(proof_id, target_state)))
        Vcxdemo.get_loop().run_until_complete(asyncio.gather(wait_for_state(proof, target_state)))

    def retrieve_proof(self, proof_id):
        proof = self.get_proof_request(proof_id)
        res = Vcxdemo.get_loop().run_until_complete(asyncio.gather(proof.get_proof(self.connection)))
        print(res[0])
        if len(res) > 0:
            return res[0]
        else:
            return None

    @staticmethod
    def lookup_schema(source_id, sequence_number):
        res = Vcxdemo.get_loop().run_until_complete(asyncio.gather(Schema.lookup(source_id, sequence_number)))
        if len(res) > 0:
            return res[0]

