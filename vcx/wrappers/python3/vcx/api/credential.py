from typing import Optional
from ctypes import *
from vcx.common import do_call, create_cb
from vcx.api.connection import Connection
from vcx.api.vcx_stateful import VcxStateful

import json
import json


class Credential(VcxStateful):

    def __init__(self, source_id: str):
        VcxStateful.__init__(self, source_id)
        self._name = source_id
        self._cred_offer = None

    def __del__(self):
        self.release()
        self.logger.debug("Deleted {} obj: {}".format(Credential, self.handle))

    @property
    def cred_offer(self):
        return self._cred_offer

    @cred_offer.setter
    def cred_offer(self, x):
        self._cred_offer = x

    @staticmethod
    async def create(source_id: str, credential_offer: str):
        constructor_params = (source_id,)

        c_source_id = c_char_p(source_id.encode('utf-8'))
        c_offer = c_char_p(json.dumps(credential_offer).encode('utf-8'))
        c_params = (c_source_id, c_offer, )

        return await Credential._create("vcx_credential_create_with_offer",
                                        constructor_params,
                                        c_params)

    @staticmethod
    async def create_with_msgid(source_id: str, connection: Connection, msg_id: str):
        credential = Credential(source_id,)

        c_source_id = c_char_p(source_id.encode('utf-8'))
        c_msg_id = c_char_p(json.dumps(msg_id).encode('utf-8'))
        c_connection_handle = c_uint32(connection.handle)

        if not hasattr(Credential.create_with_msgid, "cb"):
            Credential.create_with_msgid.cb = create_cb(CFUNCTYPE(None, c_uint32, c_uint32, c_uint32, c_char_p))

        credential.handle, cred_offer = await do_call('vcx_credential_create_with_msgid',
                                                      c_source_id,
                                                      c_connection_handle,
                                                      c_msg_id,
                                                      Credential.create_with_msgid.cb)

        credential.cred_offer = json.loads(cred_offer.decode())

        return credential

    @staticmethod
    async def deserialize(data: dict):
        credential = await Credential._deserialize("vcx_credential_deserialize",
                                                   json.dumps(data),
                                                   data.get('source_id'))
        return credential

    @staticmethod
    async def get_offers(connection: Connection) -> dict:
        if not hasattr(Credential.get_offers, "cb"):
            Credential.get_offers.cb = create_cb(CFUNCTYPE(None, c_uint32, c_uint32, c_char_p))

        c_connection_handle = c_uint32(connection.handle)

        data = await do_call('vcx_credential_get_offers',
                             c_connection_handle,
                             Credential.get_offers.cb)

        return json.loads(data.decode())

    async def serialize(self) -> dict:
        return await self._serialize(Credential, 'vcx_credential_serialize')

    async def update_state(self) -> int:
        return await self._update_state(Credential, 'vcx_credential_update_state')

    async def get_state(self) -> int:
        return await self._get_state(Credential, 'vcx_credential_get_state')

    def release(self) -> None:
        self._release(Credential, 'vcx_credential_release')

    async def send_request(self, connection: Connection, payment_handle: int):
        if not hasattr(Credential.send_request, "cb"):
            self.logger.debug("vcx_credential_send_request: Creating callback")
            Credential.send_request.cb = create_cb(CFUNCTYPE(None, c_uint32, c_uint32))

        c_credential_handle = c_uint32(self.handle)
        c_connection_handle = c_uint32(connection.handle)
        c_payment = c_uint32(payment_handle)

        await do_call('vcx_credential_send_request',
                      c_credential_handle,
                      c_connection_handle,
                      c_payment,
                      Credential.send_request.cb)

    async def get_payment_info(self):
        if not hasattr(Credential.get_payment_info, "cb"):
            self.logger.debug("vcx_credential_get_payment_info: Creating callback")
            Credential.get_payment_info.cb = create_cb(CFUNCTYPE(None, c_uint32, c_uint32, c_char_p))

        c_credential_handle = c_uint32(self.handle)
        data = await do_call('vcx_credential_get_payment_info',
                      c_credential_handle,
                      Credential.get_payment_info.cb)
        return json.loads(data.decode())

    async def get_payment_txn(self):
        if not hasattr(Credential.get_payment_txn, "cb"):
            self.logger.debug("vcx_credential_get_payment_txn: Creating callback")
            Credential.get_payment_txn.cb = create_cb(CFUNCTYPE(None, c_uint32, c_uint32, c_char_p))

        c_credential_handle = c_uint32(self.handle)

        payment_txn = await do_call('vcx_credential_get_payment_txn',
                      c_credential_handle,
                      Credential.get_payment_txn.cb)

        return json.loads(payment_txn.decode())








