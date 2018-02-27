from typing import Optional
from ctypes import *
from vcx.common import do_call, create_cb
from vcx.api.connection import Connection
from vcx.api.vcx_stateful import VcxStateful

import json


class IssuerClaim(VcxStateful):

    def __init__(self, source_id: str, attrs: dict, schema_no: int, name: str):
        VcxStateful.__init__(self, source_id)
        self._schema_no = schema_no
        self._attrs = attrs
        self._name = name

    def __del__(self):
        self.release()
        self.logger.debug("Deleted {} obj: {}".format(IssuerClaim, self.handle))

    @staticmethod
    async def create(source_id: str, attrs: dict, schema_no: int, name: str):
        attrs = {k: [v] for k, v in attrs.items()}
        constructor_params = (source_id, attrs, schema_no, name)

        c_source_id = c_char_p(source_id.encode('utf-8'))
        c_schema_no = c_uint32(schema_no)
        # default enterprise_did in config is used as issuer_did
        c_issuer_did = None
        c_data = c_char_p(json.dumps(attrs).encode('utf-8'))
        c_name = c_char_p(name.encode('utf-8'))
        c_params = (c_source_id, c_schema_no, c_issuer_did, c_data, c_name)

        return await IssuerClaim._create("vcx_issuer_create_claim",
                                         constructor_params,
                                         c_params)

    @staticmethod
    async def deserialize(data: dict):
        issuer_claim = await IssuerClaim._deserialize("vcx_issuer_claim_deserialize",
                                                      json.dumps(data),
                                                      data.get('source_id'),
                                                      data.get('claim_attributes'),
                                                      data.get('schema_seq_no'),
                                                      data.get('claim_request'))
        return issuer_claim

    async def serialize(self) -> dict:
        return await self._serialize(IssuerClaim, 'vcx_issuer_claim_serialize')

    async def update_state(self) -> int:
        return await self._update_state(IssuerClaim, 'vcx_issuer_claim_update_state')

    async def get_state(self) -> int:
        return await self._get_state(IssuerClaim, 'vcx_issuer_claim_get_state')

    def release(self) -> None:
        self._release(IssuerClaim, 'vcx_claim_issuer_release')

    async def send_offer(self, connection: Connection):
        if not hasattr(IssuerClaim.send_offer, "cb"):
            self.logger.debug("vcx_issuer_send_claim_offer: Creating callback")
            IssuerClaim.send_offer.cb = create_cb(CFUNCTYPE(None, c_uint32, c_uint32))

        c_claim_handle = c_uint32(self.handle)
        c_connection_handle = c_uint32(connection.handle)

        await do_call('vcx_issuer_send_claim_offer',
                      c_claim_handle,
                      c_connection_handle,
                      IssuerClaim.send_offer.cb)

    async def send_claim(self, connection: Connection):
        if not hasattr(IssuerClaim.send_claim, "cb"):
            self.logger.debug("vcx_issuer_send_claim: Creating callback")
            IssuerClaim.send_claim.cb = create_cb(CFUNCTYPE(None, c_uint32, c_uint32))

        c_claim_handle = c_uint32(self.handle)
        c_connection_handle = c_uint32(connection.handle)

        await do_call('vcx_issuer_send_claim',
                      c_claim_handle,
                      c_connection_handle,
                      IssuerClaim.send_claim.cb)
