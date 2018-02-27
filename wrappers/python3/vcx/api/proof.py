from ctypes import *
from vcx.common import do_call, create_cb
from vcx.api.connection import Connection
from vcx.api.vcx_stateful import VcxStateful

import json


class Proof(VcxStateful):

    def __init__(self, source_id: str):
        VcxStateful.__init__(self, source_id)
        self._proof_state = 0

    def __del__(self):
        self.release()
        self.logger.debug("Deleted {} obj: {}".format(Proof, self.handle))

    @property
    def proof_state(self):
        return self._proof_state

    @proof_state.setter
    def proof_state(self, x):
        self._proof_state = x

    @staticmethod
    async def create(source_id: str, name: str, requested_attrs: list):
        constructor_params = (source_id,)

        c_source_id = c_char_p(source_id.encode('utf-8'))
        c_name = c_char_p(name.encode('utf-8'))
        c_req_predicates = c_char_p('[]'.encode('utf-8'))
        c_req_attrs = c_char_p(json.dumps(requested_attrs).encode('utf-8'))
        c_params = (c_source_id, c_req_attrs, c_req_predicates, c_name)

        return await Proof._create("vcx_proof_create",
                                   constructor_params,
                                   c_params)

    @staticmethod
    async def deserialize(data: dict):
        return await Proof._deserialize("vcx_proof_deserialize",
                                        json.dumps(data),
                                        data.get('source_id'))

    async def serialize(self) -> dict:
        return await self._serialize(Proof, 'vcx_proof_serialize')

    async def update_state(self) -> int:
        return await self._update_state(Proof, 'vcx_proof_update_state')

    async def get_state(self) -> int:
        return await self._get_state(Proof, 'vcx_proof_get_state')

    def release(self) -> None:
        self._release(Proof, 'vcx_proof_release')

    async def request_proof(self, connection: Connection):
        if not hasattr(Proof.request_proof, "cb"):
            self.logger.debug("vcx_proof_send_request: Creating callback")
            Proof.request_proof.cb = create_cb(CFUNCTYPE(None, c_uint32, c_uint32))

        c_proof_handle = c_uint32(self.handle)
        c_connection_handle = c_uint32(connection.handle)

        await do_call('vcx_proof_send_request',
                      c_proof_handle,
                      c_connection_handle,
                      Proof.request_proof.cb)

    async def get_proof(self, connection: Connection) -> list:
        if not hasattr(Proof.get_proof, "cb"):
            self.logger.debug("vcx_get_proof: Creating callback")
            Proof.get_proof.cb = create_cb(CFUNCTYPE(None, c_uint32, c_uint32, c_uint32, c_char_p))

        c_proof_handle = c_uint32(self.handle)
        c_connection_handle = c_uint32(connection.handle)

        proof_state, proof = await do_call('vcx_get_proof',
                                           c_proof_handle,
                                           c_connection_handle,
                                           Proof.get_proof.cb)
        self.proof_state = proof_state
        return json.loads(proof.decode())
