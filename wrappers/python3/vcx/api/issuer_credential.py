from typing import Optional
from ctypes import *
from vcx.common import do_call, create_cb
from vcx.api.connection import Connection
from vcx.api.vcx_stateful import VcxStateful

import json


class IssuerCredential(VcxStateful):

    def __init__(self, source_id: str, attrs: dict, cred_def_id: str, name: str, price: float):
        VcxStateful.__init__(self, source_id)
        self._cred_def_id = cred_def_id
        self._attrs = attrs
        self._name = name
        self._price = price

    def __del__(self):
        self.release()
        self.logger.debug("Deleted {} obj: {}".format(IssuerCredential, self.handle))

    @staticmethod
    async def create(source_id: str, attrs: dict, cred_def_id: str, name: str, price: int):
        attrs = {k: [v] for k, v in attrs.items()}
        constructor_params = (source_id, attrs, cred_def_id, name, price)

        c_source_id = c_char_p(source_id.encode('utf-8'))
        c_cred_def_id = c_char_p(cred_def_id.encode('utf-8'))
        c_price = c_uint64(price)
        # default institution_did in config is used as issuer_did
        c_issuer_did = None
        c_data = c_char_p(json.dumps(attrs).encode('utf-8'))
        c_name = c_char_p(name.encode('utf-8'))
        c_params = (c_source_id, c_cred_def_id, c_issuer_did, c_data, c_name, c_price)

        return await IssuerCredential._create("vcx_issuer_create_credential",
                                         constructor_params,
                                         c_params)

    @staticmethod
    async def deserialize(data: dict):
        issuer_credential = await IssuerCredential._deserialize("vcx_issuer_credential_deserialize",
                                                      json.dumps(data),
                                                      data.get('source_id'),
                                                      data.get('price'),
                                                      data.get('credential_attributes'),
                                                      data.get('schema_seq_no'),
                                                      data.get('credential_request'))
        return issuer_credential

    async def serialize(self) -> dict:
        return await self._serialize(IssuerCredential, 'vcx_issuer_credential_serialize')

    async def update_state(self) -> int:
        return await self._update_state(IssuerCredential, 'vcx_issuer_credential_update_state')

    async def get_state(self) -> int:
        return await self._get_state(IssuerCredential, 'vcx_issuer_credential_get_state')

    def release(self) -> None:
        self._release(IssuerCredential, 'vcx_issuer_credential_release')

    async def send_offer(self, connection: Connection):
        if not hasattr(IssuerCredential.send_offer, "cb"):
            self.logger.debug("vcx_issuer_send_credential_offer: Creating callback")
            IssuerCredential.send_offer.cb = create_cb(CFUNCTYPE(None, c_uint32, c_uint32))

        c_credential_handle = c_uint32(self.handle)
        c_connection_handle = c_uint32(connection.handle)

        await do_call('vcx_issuer_send_credential_offer',
                      c_credential_handle,
                      c_connection_handle,
                      IssuerCredential.send_offer.cb)

    async def send_credential(self, connection: Connection):
        if not hasattr(IssuerCredential.send_credential, "cb"):
            self.logger.debug("vcx_issuer_send_credential: Creating callback")
            IssuerCredential.send_credential.cb = create_cb(CFUNCTYPE(None, c_uint32, c_uint32))

        c_credential_handle = c_uint32(self.handle)
        c_connection_handle = c_uint32(connection.handle)

        await do_call('vcx_issuer_send_credential',
                      c_credential_handle,
                      c_connection_handle,
                      IssuerCredential.send_credential.cb)
