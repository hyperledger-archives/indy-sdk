from typing import Optional
from ctypes import *
from vcx.common import do_call, create_cb
from vcx.api.connection import Connection
from vcx.api.vcx_stateful import VcxStateful

import json


class DisclosedProof(VcxStateful):

    def __init__(self, source_id: str):
        VcxStateful.__init__(self, source_id)
        self._name = source_id
        self._proof_req = None

    def __del__(self):
        self.release()
        self.logger.debug("Deleted {} obj: {}".format(DisclosedProof, self.handle))

    @property
    def proof_request(self):
        return self._proof_req

    @proof_request.setter
    def proof_request(self, x):
        self._proof_req = x

    @staticmethod
    async def create(source_id: str, proof_request: str):
        constructor_params = (source_id,)

        c_source_id = c_char_p(source_id.encode('utf-8'))
        c_proof_request = c_char_p(json.dumps(proof_request).encode('utf-8'))
        c_params = (c_source_id, c_proof_request, )

        return await DisclosedProof._create("vcx_disclosed_proof_create_with_request",
                                   constructor_params,
                                   c_params)

    @staticmethod
    async def create_with_msgid(source_id: str, connection: Connection, msg_id: str):
        proof = DisclosedProof(source_id)

        c_source_id = c_char_p(source_id.encode('utf-8'))
        c_msg_id = c_char_p(json.dumps(msg_id).encode('utf-8'))
        c_connection_handle = c_uint32(connection.handle)

        if not hasattr(DisclosedProof.create_with_msgid, "cb"):
            DisclosedProof.create_with_msgid.cb = create_cb(CFUNCTYPE(None, c_uint32, c_uint32, c_uint32, c_char_p))

        proof.handle, proof_req = await do_call('vcx_disclosed_proof_create_with_msgid',
                                                c_source_id,
                                                c_connection_handle,
                                                c_msg_id,
                                                DisclosedProof.create_with_msgid.cb)

        proof.proof_request = json.loads(proof_req.decode())

        return proof

    @staticmethod
    async def deserialize(data: dict):
        disclosed_proof = await DisclosedProof._deserialize("vcx_disclosed_proof_deserialize",
                                                      json.dumps(data),
                                                      data.get('source_id'))
        return disclosed_proof

    @staticmethod
    async def get_requests(connection: Connection) -> dict:
        if not hasattr(DisclosedProof.get_requests, "cb"):
            DisclosedProof.get_requests.cb = create_cb(CFUNCTYPE(None, c_uint32, c_uint32, c_char_p))

        c_connection_handle = c_uint32(connection.handle)

        data = await do_call('vcx_disclosed_proof_get_requests',
                      c_connection_handle,
                      DisclosedProof.get_requests.cb)

        return json.loads(data.decode())

    async def serialize(self) -> dict:
        return await self._serialize(DisclosedProof, 'vcx_disclosed_proof_serialize')

    async def update_state(self) -> int:
        return await self._update_state(DisclosedProof, 'vcx_disclosed_proof_update_state')

    async def get_state(self) -> int:
        return await self._get_state(DisclosedProof, 'vcx_disclosed_proof_get_state')

    def release(self) -> None:
        self._release(DisclosedProof, 'vcx_disclosed_proof_release')

    async def get_creds(self) -> dict:
        if not hasattr(DisclosedProof.get_creds, "cb"):
            self.logger.debug("vcx_disclosed_proof_retrieve_credentials: Creating callback")
            DisclosedProof.send_proof.cb = create_cb(CFUNCTYPE(None, c_uint32, c_uint32, c_char_p))

        c_disclosed_proof_handle = c_uint32(self.handle)

        data = await do_call('vcx_disclosed_proof_retrieve_credentials',
                             c_disclosed_proof_handle,
                             DisclosedProof.send_proof.cb)
        return json.loads(data.decode())

    async def send_proof(self, connection: Connection):
        if not hasattr(DisclosedProof.send_proof, "cb"):
            self.logger.debug("vcx_disclosed_proof_send_proof: Creating callback")
            DisclosedProof.send_proof.cb = create_cb(CFUNCTYPE(None, c_uint32, c_uint32))

        c_disclosed_proof_handle = c_uint32(self.handle)
        c_connection_handle = c_uint32(connection.handle)

        await do_call('vcx_disclosed_proof_send_proof',
                      c_disclosed_proof_handle,
                      c_connection_handle,
                      DisclosedProof.send_proof.cb)

    async def generate_proof(self, selected_creds: dict, self_attested_attrs: dict):
        if not hasattr(DisclosedProof.send_proof, "cb"):
            self.logger.debug("vcx_disclosed_proof_generate_proof: Creating callback")
            DisclosedProof.send_proof.cb = create_cb(CFUNCTYPE(None, c_uint32, c_uint32))

        c_disclosed_proof_handle = c_uint32(self.handle)
        c_selected_creds = c_char_p(json.dumps(selected_creds).encode('utf-8'))
        c_self_attested_attrs = c_char_p(json.dumps(self_attested_attrs).encode('utf-8'))

        await do_call('vcx_disclosed_proof_generate_proof',
                      c_disclosed_proof_handle,
                      c_selected_creds,
                      c_self_attested_attrs,
                      DisclosedProof.send_proof.cb)
