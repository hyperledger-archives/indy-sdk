from typing import Optional
from ctypes import *
from vcx.common import do_call, create_cb
from vcx.api.connection import Connection
from vcx.api.vcx_stateful import VcxStateful

import json


class Claim(VcxStateful):

    def __init__(self, source_id: str):
        VcxStateful.__init__(self, source_id)
        self._name = source_id

    def __del__(self):
        self.release()
        self.logger.debug("Deleted {} obj: {}".format(Claim, self.handle))

    @staticmethod
    async def create(source_id: str, claim_offer: dict):
        constructor_params = source_id

        c_source_id = c_char_p(source_id.encode('utf-8'))
        c_offer = c_char_p(json.dumps(claim_offer).encode('utf-8'))
        c_params = (c_source_id, c_offer)

        return await Claim._create("vcx_claim_create_with_offer",
                                   constructor_params,
                                   c_params)

    @staticmethod
    async def deserialize(data: dict):
        claim = await Claim._deserialize("vcx_claim_deserialize",
                                                      json.dumps(data),
                                                      data.get('source_id'))
        return claim

    @staticmethod
    async def get_offers(connection: Connection):
        if not hasattr(Claim.get_offers, "cb"):
            Claim.get_offers.cb = create_cb(CFUNCTYPE(None, c_uint32, c_uint32))

        c_connection_handle = c_uint32(connection.handle)

        await do_call('vcx_claim_get_offers',
                      c_connection_handle,
                      Claim.get_offers.cb)

    async def serialize(self) -> dict:
        return await self._serialize(Claim, 'vcx_claim_serialize')

    async def update_state(self) -> int:
        return await self._update_state(Claim, 'vcx_claim_update_state')

    async def get_state(self) -> int:
        return await self._get_state(Claim, 'vcx_claim_get_state')

    def release(self) -> None:
        self._release(Claim, 'vcx_claim_release')

    async def send_request(self, connection: Connection):
        if not hasattr(Claim.send_request, "cb"):
            self.logger.debug("vcx_claim_send_request: Creating callback")
            Claim.send_request.cb = create_cb(CFUNCTYPE(None, c_uint32, c_uint32))

        c_claim_handle = c_uint32(self.handle)
        c_connection_handle = c_uint32(connection.handle)

        await do_call('vcx_claim_send_request',
                      c_claim_handle,
                      c_connection_handle,
                      Claim.send_request.cb)
