from typing import Optional
from ctypes import *
from vcx.common import do_call, create_cb
from vcx.api.connection import Connection
from vcx.api.vcx_stateful import VcxStateful

import json

class IssuerCredential(VcxStateful):
    """Class representing an Issuer Credential"""

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
    async def create(source_id: str, attrs: dict, cred_def_handle: int, name: str, price: str):
        """
            Creates a Class representing an Issuer Credential
            :param source_id: Tag associated by user of sdk
            :param attrs: attributes that will form the credential
            :param cred_def_handle: Handle from previously created credential def object
            :param name: Name given to the Credential
            :param price: Price, in tokens, required as payment for the issuance of the credential.

            Example:
            source_id = '1'
            cred_def_handle = 1
            attrs = {'key': 'value', 'key2': 'value2', 'key3': 'value3'}
            name = 'Credential Name'
            issuer_did = '8XFh8yBzrpJQmNyZzgoTqB'
            phone_number = '8019119191'
            price = 1
            issuer_credential = await IssuerCredential.create(source_id, attrs, cred_def_handle, name, price)
        """
        constructor_params = (source_id, attrs, cred_def_handle, name, price)

        c_source_id = c_char_p(source_id.encode('utf-8'))
        c_cred_def_handle = c_uint32(cred_def_handle)
        c_price = c_char_p(price.encode('utf-8'))
        # default institution_did in config is used as issuer_did
        c_issuer_did = None
        c_data = c_char_p(json.dumps(attrs).encode('utf-8'))
        c_name = c_char_p(name.encode('utf-8'))
        c_params = (c_source_id, c_cred_def_handle, c_issuer_did, c_data, c_name, c_price)

        return await IssuerCredential._create("vcx_issuer_create_credential",
                                              constructor_params,
                                              c_params)

    @staticmethod
    async def deserialize(data: dict):
        """
            Creates IssuerCredential object from a dict.
            :param data: dict representing a serialized IssuerCredential Object
            :return: IssuerCredential object

            Example:
            source_id = '1'
            cred_def_id = 'cred_def_id1'
            attrs = {'key': 'value', 'key2': 'value2', 'key3': 'value3'}
            name = 'Credential Name'
            issuer_did = '8XFh8yBzrpJQmNyZzgoTqB'
            phone_number = '8019119191'
            price = 1
            issuer_credential = await IssuerCredential.create(source_id, attrs, cred_def_id, name, price)
            data = await issuer_credential.serialize()
            issuer_credential2 = await IssuerCredential.deserialize(data)
        """
        issuer_credential = await IssuerCredential._deserialize("vcx_issuer_credential_deserialize",
                                                      json.dumps(data),
                                                      data.get('data').get('source_id'),
                                                      data.get('data').get('price'),
                                                      data.get('data').get('credential_attributes'),
                                                      data.get('data').get('schema_seq_no'),
                                                      data.get('data').get('credential_request'))
        return issuer_credential

    async def serialize(self) -> dict:
        """
            Serializes a issuer credential.

            Example:
            source_id = '1'
            cred_def_id = 'cred_def_id1'
            attrs = {'key': 'value', 'key2': 'value2', 'key3': 'value3'}
            name = 'Credential Name'
            issuer_did = '8XFh8yBzrpJQmNyZzgoTqB'
            phone_number = '8019119191'
            price = 1
            issuer_credential = await IssuerCredential.create(source_id, attrs, cred_def_id, name, price)
            data = await issuer_credential.serialize()
        :return: dictionary representing the serialized object
        """
        return await self._serialize(IssuerCredential, 'vcx_issuer_credential_serialize')

    async def update_state(self) -> int:
        """
        Communicates with the agent service for polling and setting the state of the entity.
        Example:
        issuer_credential = await IssuerCredential.create(source_id, attrs, cred_def_id, name, price)
        issuer_credential.update_state()
        :return:
        """
        return await self._update_state(IssuerCredential, 'vcx_issuer_credential_update_state')

    async def get_state(self) -> int:
        """
        Gets the state of the entity.
        Example:
        issuer_credential = await IssuerCredential.create(source_id, attrs, cred_def_id, name, price)
        issuer_credential.update_state()
        :return: State of the Object
        """
        return await self._get_state(IssuerCredential, 'vcx_issuer_credential_get_state')

    def release(self) -> None:
        """
        Used to release memory associated with this object, used by the c library.
        :return:
        """
        self._release(IssuerCredential, 'vcx_issuer_credential_release')

    async def send_offer(self, connection: Connection):
        """
        Sends an offer to a prover.  Once accepted, a request will be recieved.
        :param connection: vcx.api.connection.Connection
        :return: None

        Example:
        source_id = '1'
        cred_def_id = 'cred_def_id1'
        attrs = {'key': 'value', 'key2': 'value2', 'key3': 'value3'}
        name = 'Credential Name'
        issuer_did = '8XFh8yBzrpJQmNyZzgoTqB'
        phone_number = '8019119191'
        price = 1
        issuer_credential = await IssuerCredential.create(source_id, attrs, cred_def_id, name, price)
        connection = await Connection.create(source_id)
        issuer_credential.send_offer(connection)
        """
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
        """
        Sends the credential to the end user (prover).
        :param connection: Connection Object
        :return: None
            Example:
            credential.send_credential(connection)
        """
        if not hasattr(IssuerCredential.send_credential, "cb"):
            self.logger.debug("vcx_issuer_send_credential: Creating callback")
            IssuerCredential.send_credential.cb = create_cb(CFUNCTYPE(None, c_uint32, c_uint32))

        c_credential_handle = c_uint32(self.handle)
        c_connection_handle = c_uint32(connection.handle)

        await do_call('vcx_issuer_send_credential',
                      c_credential_handle,
                      c_connection_handle,
                      IssuerCredential.send_credential.cb)

    async def revoke_credential(self):
        """
        Revokes a credential.
        :return: None
            Example:
            credential.revoke_credential()
        """
        if not hasattr(IssuerCredential.revoke_credential, "cb"):
            self.logger.debug("vcx_issuer_revoke_credential: Creating callback")
            IssuerCredential.revoke_credential.cb = create_cb(CFUNCTYPE(None, c_uint32, c_uint32))

        c_credential_handle = c_uint32(self.handle)

        await do_call('vcx_issuer_revoke_credential',
                      c_credential_handle,
                      IssuerCredential.revoke_credential.cb)

    async def get_payment_txn(self):
        """
        Retrieve Payment Transaction that was used to pay for this Credential
        Example:
        txn = credential.get_payment_txn()
        :return:
        """
        if not hasattr(IssuerCredential.get_payment_txn, "cb"):
            self.logger.debug("vcx_issuer_credential_get_payment_txn: Creating callback")
            IssuerCredential.get_payment_txn.cb = create_cb(CFUNCTYPE(None, c_uint32, c_uint32, c_char_p))

        c_credential_handle = c_uint32(self.handle)

        payment_txn = await do_call('vcx_issuer_credential_get_payment_txn',
                      c_credential_handle,
                      IssuerCredential.get_payment_txn.cb)

        return json.loads(payment_txn.decode())

