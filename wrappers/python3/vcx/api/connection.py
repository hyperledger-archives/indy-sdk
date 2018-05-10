from typing import Optional
from ctypes import *
from vcx.common import do_call, create_cb
from vcx.api.vcx_stateful import VcxStateful

import json


class Connection(VcxStateful):

    def __init__(self, source_id: str):
        VcxStateful.__init__(self, source_id)

    def __del__(self):
        self.release()
        self.logger.debug("Deleted {} obj: {}".format(Connection, self.handle))

    @staticmethod
    async def create(source_id: str):
        constructor_params = (source_id,)

        c_source_id = c_char_p(source_id.encode('utf-8'))
        c_params = (c_source_id,)

        return await Connection._create( "vcx_connection_create",
                                        constructor_params,
                                        c_params)

    @staticmethod
    async def create_with_details(source_id: str, invite_details: str):
        constructor_params = (source_id,)

        c_source_id = c_char_p(source_id.encode('utf-8'))
        c_invite_details = c_char_p(invite_details.encode('utf-8'))

        c_params = (c_source_id, c_invite_details, )

        return await Connection._create( "vcx_connection_create_with_invite",
                                        constructor_params,
                                        c_params)

    @staticmethod
    async def deserialize(data: dict):
        return await Connection._deserialize("vcx_connection_deserialize",
                                             json.dumps(data),
                                             data.get('source_id'))

    async def connect(self, phone_number: Optional[str]) -> str:
        if not hasattr(Connection.connect, "cb"):
            self.logger.debug("vcx_connection_connect: Creating callback")
            Connection.connect.cb = create_cb(CFUNCTYPE(None, c_uint32, c_uint32, c_char_p))

        c_connection_handle = c_uint32(self.handle)
        connection_data = {'connection_type': 'SMS', 'phone': phone_number} if phone_number \
            else {'connection_type': 'QR'}
        c_connection_data = c_char_p(json.dumps(connection_data).encode('utf-8'))
        invite_details = await do_call('vcx_connection_connect',
                                       c_connection_handle,
                                       c_connection_data,
                                       Connection.connect.cb)
        return invite_details

    async def _delete(self):
        if not hasattr(Connection._delete, "cb"):
            Connection._delete.cb = create_cb(CFUNCTYPE(None, c_uint32, c_uint32))

        c_connection_handle = c_uint32(self.handle)
        return await do_call('vcx_connection_delete_connection', c_connection_handle, Connection._delete.cb)

    async def serialize(self) -> dict:
        return await self._serialize(Connection, 'vcx_connection_serialize')

    async def update_state(self) -> int:
        return await self._update_state(Connection, 'vcx_connection_update_state')

    async def get_state(self) -> int:
        return await self._get_state(Connection, 'vcx_connection_get_state')

    def release(self) -> None:
        self._release(Connection, 'vcx_connection_release')

    async def delete(self):
        await self._delete()

    async def invite_details(self, abbreviated: bool) -> dict:
        if not hasattr(Connection.invite_details, "cb"):
            self.logger.debug("vcx_connection_invite_details: Creating callback")
            Connection.invite_details.cb = create_cb(CFUNCTYPE(None, c_uint32, c_uint32, c_char_p))

        c_connection_handle = c_uint32(self.handle)
        c_abbreviated = c_bool(abbreviated)

        details = await do_call('vcx_connection_invite_details',
                                c_connection_handle,
                                c_abbreviated,
                                Connection.invite_details.cb)

        return json.loads(details.decode())
