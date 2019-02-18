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
        Create a connection object with a provided invite, represents a single endpoint and can be used for sending and receiving
        credentials and proofs

        Invite details are provided by the entity offering a connection and generally pulled from a provided QRCode.
        :param source_id: Institution's unique ID for the connection
        :param invite_details: A string representing a json object which is provided by an entity that wishes to make a connection.
        Example:
        connection2 = await Connection.create_with_details('MyBank', invite_details)
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
        Example:
        data = await connection1.serialize()
        connection2 = await Connection.deserialize(data)
        :return: A re-instantiated object
        """
        return await Connection._deserialize("vcx_connection_deserialize",
                                             json.dumps(data),
                                             data.get('source_id'))

    async def connect(self, options: str) -> str:
        """
        Connect securely and privately to the endpoint represented by the object.

        :param options: detailed connection options
        Example options:
        {"connection_type":"SMS","phone":"5555555555","use_public_did":true}
        or:
        {"connection_type":"QR"}
        Example code:
        connection = await Connection.create('Sally')
        invite_details = await connection.connect('{"connection_type":"QR"}')
        :return: the invite details sent via SMS or ready to be sent via some other mechanism (QR for example)
        """
        if not hasattr(Connection.connect, "cb"):
            self.logger.debug("vcx_connection_connect: Creating callback")
            Connection.connect.cb = create_cb(CFUNCTYPE(None, c_uint32, c_uint32, c_char_p))

        c_connection_handle = c_uint32(self.handle)
        c_connection_data = c_char_p(options.encode('utf-8'))
        invite_details = await do_call('vcx_connection_connect',
                                       c_connection_handle,
                                       c_connection_data,
                                       Connection.connect.cb)
        return invite_details

    async def send_message(self, msg: str, msg_type: str, msg_title: str) -> str:
        """
            Send a generic message to the connection
            :param msg:
            :param msg_type:
            :param msg_title:
            :return:
            """
        if not hasattr(Connection.send_message, "cb"):
            self.logger.debug("vcx_connection_send_message: Creating callback")
            Connection.send_message.cb = create_cb(CFUNCTYPE(None, c_uint32, c_uint32, c_char_p))

        c_connection_handle = c_uint32(self.handle)
        c_msg = c_char_p(msg.encode('utf-8'))
        c_msg_type = c_char_p(msg_type.encode('utf-8'))
        c_msg_title = c_char_p(msg_title.encode('utf-8'))

        result = await do_call('vcx_connection_send_message',
                               c_connection_handle,
                               c_msg,
                               c_msg_type,
                               c_msg_title,
                               Connection.send_message.cb)

        self.logger.debug("vcx_connection_send_message completed")
        return result

    async def sign_data(self, msg: bytes) -> bytes:
        """
        Sign data using connection's pairwise key
        :param msg:
        :return: signature
        """

        def transform_cb(arr_ptr: POINTER(c_uint8), arr_len: c_uint32):
            return bytes(arr_ptr[:arr_len]),

        if not hasattr(Connection.sign_data, "cb"):
            self.logger.debug("vcx_connection_sign_data: Creating callback")
            Connection.sign_data.cb = create_cb(CFUNCTYPE(None, c_uint32, c_uint32, POINTER(c_uint8), c_uint32), transform_cb)

        c_connection_handle = c_uint32(self.handle)
        c_msg_len = c_uint32(len(msg))

        result = await do_call('vcx_connection_sign_data',
                               c_connection_handle,
                               msg,
                               c_msg_len,
                               Connection.sign_data.cb)

        self.logger.debug("vcx_connection_sign_data completed")
        return result

    async def verify_signature(self, msg: bytes, signature: bytes) -> bool:
        """
        Verification the signature of a msg
        :param msg:
        :param signature:
        :return: bool
        """
        if not hasattr(Connection.verify_signature, "cb"):
            self.logger.debug("vcx_connection_verify_signature: Creating callback")
            Connection.verify_signature.cb = create_cb(CFUNCTYPE(None, c_uint32, c_uint32, c_bool))

        c_connection_handle = c_uint32(self.handle)
        c_msg_len = c_uint32(len(msg))
        c_signature_len = c_uint32(len(signature))

        result = await do_call('vcx_connection_verify_signature',
                               c_connection_handle,
                               msg,
                               c_msg_len,
                               signature,
                               c_signature_len,
                               Connection.verify_signature.cb)

        self.logger.debug("vcx_connection_verify_signature completed")
        return result

    async def _delete(self):
        if not hasattr(Connection._delete, "cb"):
            Connection._delete.cb = create_cb(CFUNCTYPE(None, c_uint32, c_uint32))

        c_connection_handle = c_uint32(self.handle)
        return await do_call('vcx_connection_delete_connection', c_connection_handle, Connection._delete.cb)

    async def serialize(self) -> dict:
        """
        Serialize the object for storage
        Example:
        data = await connection1.serialize()
        :return: serialized object
        """
        return await self._serialize(Connection, 'vcx_connection_serialize')

    async def update_state(self) -> int:
        """
        Query the agency for the current state of the connection.  Used to determine whether the connection
        has been accepted by both endpoints.
        Example:
        connection = await Connection.create(source_id)
        assert await connection.update_state() == State.Initialized
        :return: Current state of the connection
        """
        return await self._update_state(Connection, 'vcx_connection_update_state')

    async def get_state(self) -> int:
        """
        Returns the current internal state of the connection.  Does NOT query agency for state updates.
        Example:
        connection = await Connection.create(source_id)
        assert await connection.get_state() == State.Initialized
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

        Example:
        connection = await Connection.create(source_id)
        await connection.delete()
        :return: None
        """
        await self._delete()

    async def invite_details(self, abbreviated: bool) -> dict:
        """
        Get the invite details that were sent or can be sent to the endpoint.

        :param abbreviated: abbreviate invite details or not
        Example:
        phone_number = '8019119191'
        connection = await Connection.create('foobar123')
        invite_details = await connection.connect(phone_number)
        inivte_details_again = await connection.invite_details()
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
