"""
The basic object of the VCX API.  Represents a pairwise relationship with another identity owner.  Once the
relationship, or connection, is established communication can happen securely and privately.  Credentials and
proofs are exchanged using this object.

TODO: document attributes
"""
from typing import Optional
from ctypes import *
from vcx.common import do_call, create_cb
from vcx.api.vcx_stateful import VcxStateful

import json


class Connection(VcxStateful):
    """
    The basic object of the VCX API.  Represents a pairwise relationship with another identity owner.  Once the
    relationship, or connection, is established communication can happen securely and privately.  Credentials and
    proofs are exchanged using this object.

    TODO: document attributes
    """

    def __init__(self, source_id: str):
        VcxStateful.__init__(self, source_id)

    def __del__(self):
        self.release()
        self.logger.debug("Deleted {} obj: {}".format(Connection, self.handle))

    @staticmethod
    async def create(source_id: str):
        """
        Create a connection object, represents a single endpoint and can be used for sending and receiving
        credentials and proofs

        :param source_id: Institution's unique ID for the connection
        :return: connection object
        Example:
        connection = await Connection.create(source_id)
        """
        constructor_params = (source_id,)

        c_source_id = c_char_p(source_id.encode('utf-8'))
        c_params = (c_source_id,)

        return await Connection._create( "vcx_connection_create",
                                        constructor_params,
                                        c_params)

    @staticmethod
    async def create_with_details(source_id: str, invite_details: str):
        """
        Create a connection object with an invite, represents a single endpoint and can be used for sending and receiving
        credentials and proofs

        :param source_id: Institution's unique ID for the connection
        :param invite_details:
        :return: connection object
        """
        constructor_params = (source_id,)

        c_source_id = c_char_p(source_id.encode('utf-8'))
        c_invite_details = c_char_p(invite_details.encode('utf-8'))

        c_params = (c_source_id, c_invite_details, )

        return await Connection._create( "vcx_connection_create_with_invite",
                                        constructor_params,
                                        c_params)

    @staticmethod
    async def deserialize(data: dict):
        """
        Create the object from a previously serialized object.

        :param data: The output of the "serialize" call
        :return: A re-instantiated object
        """
        return await Connection._deserialize("vcx_connection_deserialize",
                                             json.dumps(data),
                                             data.get('source_id'))

    async def connect(self, phone_number: Optional[str]) -> str:
        """
        Connect securely and privately to the endpoint represented by the object.

        :param phone_number: optional phone number that will receive SMS with invite details
        :return: the invite details sent via SMS or ready to be sent via some other mechanism (QR for example)
        """
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
        """
        Serialize the object for storage

        :return: serialized object
        """
        return await self._serialize(Connection, 'vcx_connection_serialize')

    async def update_state(self) -> int:
        """
        Query the agency for the current state of the connection.  Used to determine whether the connection
        has been accepted by both endpoints.

        :return: Current state of the connection
        """
        return await self._update_state(Connection, 'vcx_connection_update_state')

    async def get_state(self) -> int:
        """
        Returns the current internal state of the connection.  Does NOT query agency for state updates.

        :return:  Current internal state of the connection
        """
        return await self._get_state(Connection, 'vcx_connection_get_state')

    def release(self) -> None:
        """
        destroy the object and release any memory associated with it

        :return: None
        """
        self._release(Connection, 'vcx_connection_release')

    async def delete(self):
        """
        Delete the object from the agency and release any memory associated with it.

        NOTE: This eliminates the connection and any ability to use it for any communication.

        :return: None
        """
        await self._delete()

    async def invite_details(self, abbreviated: bool) -> dict:
        """
        Get the invite details that were sent or can be sent to the endpoint.

        :param abbreviated: abbreviate invite details or not
        :return: JSON of invite_details sent to connection
        """
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
