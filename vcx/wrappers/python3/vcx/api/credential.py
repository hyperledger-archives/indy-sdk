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
        """
        Creates a credential with an offer.
        :param source_id: user defined id of object.
        :param credential_offer: JSON string representing the offer used as the basis of creation.
        :return: A created credential
        Example:
        offer = [{
           "msg_type": "CLAIM_OFFER",
           "version": "0.1",
           "to_did": "8XFh8yBzrpJQmNyZzgoTqB",
           "from_did": "8XFh8yBzrpJQmNyZzgoTqB",
           "libindy_offer": '{}',
           "credential_attrs": {
              "address1": [
                 "101 Tela Lane"
              ],
              "address2": [
                 "101 Wilson Lane"
              ],
              "city": [
                 "SLC"
              ],
              "state": [
                 "UT"
              ],
              "zip": [
                 "87121"
              ]
           },
           "schema_seq_no": 1487,
           "cred_def_id": "id1",
           "claim_name": "Credential",
           "claim_id": "defaultCredentialId",
           "msg_ref_id": None,
        }]
        credential = await Credential.create(source_id, offer)
        """
        constructor_params = (source_id,)

        c_source_id = c_char_p(source_id.encode('utf-8'))
        c_offer = c_char_p(json.dumps(credential_offer).encode('utf-8'))
        c_params = (c_source_id, c_offer, )

        return await Credential._create("vcx_credential_create_with_offer",
                                        constructor_params,
                                        c_params)

    @staticmethod
    async def create_with_msgid(source_id: str, connection: Connection, msg_id: str):
        """
        Create a credential based off of a known message id for a given connection.
        :param source_id: user defined id of object.
        :param connection: connection handle of connection to receive offer from
        :param msg_id: message id
        :return: A created credential
        Example:
        credential = await Credential.create_with_msgid(source_id, connection, msg_id)
        assert await credential.get_state() == State.RequestReceived
        """
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
        """
        Create a credential object from a previously serialized credential object
        :param data: JSON data from a serialized object.
        :return: A created credential
        Example:
        credential = await Credential.create(source_id, offer)
        data = await credential.serialize()
        credential2 = await Credential.deserialize(data)
        """

        credential = await Credential._deserialize("vcx_credential_deserialize",
                                                   json.dumps(data),
                                                   data.get('data').get('source_id'))
        return credential

    @staticmethod
    async def get_offers(connection: Connection) -> dict:
        """
        Retrieves all pending credential offers for a given connection.
        :param connection: A connection handle
        :return: A list of dictionary objects representing offers from a given connection.
        Example:
        credential = await Credential.create_with_msgid(source_id, connection, msg_id)
        offers = await credential.get_offers(connection)
        """
        if not hasattr(Credential.get_offers, "cb"):
            Credential.get_offers.cb = create_cb(CFUNCTYPE(None, c_uint32, c_uint32, c_char_p))

        c_connection_handle = c_uint32(connection.handle)

        data = await do_call('vcx_credential_get_offers',
                             c_connection_handle,
                             Credential.get_offers.cb)

        return json.loads(data.decode())

    async def serialize(self) -> dict:
        """
        Serializes the crednetial object for storage and later deserialization.
        :return: dict representing the object
        Example:
        credential = await Credential.create_with_msgid(source_id, connection, msg_id)
        data = credential.serialzie()
        """
        return await self._serialize(Credential, 'vcx_credential_serialize')

    async def update_state(self) -> int:
        """
        Communicates with the agent service for polling and setting the state of the entity.
        Example:
        credential = await Credential.create(source_id, offer)
        credential.update_state()
        :return:
        """
        return await self._update_state(Credential, 'vcx_credential_update_state')

    async def get_state(self) -> int:
        """
        Gets the state of the entity.
        :return: state of the object
        Example:
        credential = await Credential.create(source_id, offer)
        credential.update_state()
        state = credential.get_state()
        """
        return await self._get_state(Credential, 'vcx_credential_get_state')

    def release(self) -> None:
        """
        Used to release memory associated with this object, used by the c library.
        :return:
        """
        self._release(Credential, 'vcx_credential_release')

    async def send_request(self, connection: Connection, payment_handle: int):
        """
        Approves the credential offer and submits a credential request. The result will be a credential stored in the prover's wallet.
        :param connection: connection to submit request from
        :param payment_handle: currently unused
        :return:
        Example:
        connection = await Connection.create(source_id)
        await connection.connect(phone_number)
        credential = await Credential.create(source_id, offer)
        await credential.send_request(connection, 0)
        """
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
        """
        Retrieve Payment Transaction Information for this Credential. Typically this will include
        how much payment is requried by the issuer, which needs to be provided by the prover, before the issuer will
        issue the credential to the prover. Ideally a prover would want to know how much payment is being asked before
        submitting the credential request (which triggers the payment to be made).
        Example:
        info = credential.get_payment_info()
        :return:
        """
        if not hasattr(Credential.get_payment_info, "cb"):
            self.logger.debug("vcx_credential_get_payment_info: Creating callback")
            Credential.get_payment_info.cb = create_cb(CFUNCTYPE(None, c_uint32, c_uint32, c_char_p))

        c_credential_handle = c_uint32(self.handle)
        data = await do_call('vcx_credential_get_payment_info',
                      c_credential_handle,
                      Credential.get_payment_info.cb)
        return json.loads(data.decode())

    async def get_payment_txn(self):
        """
        Retirieve the payment transaction associated with this credential. This can be used to get the txn that
        was used to pay the issuer from the prover.  This could be considered a receipt of payment from the payer to
        the issuer.
        :return:
        Example:
        txn = credential.get_payment_txn()
        """
        if not hasattr(Credential.get_payment_txn, "cb"):
            self.logger.debug("vcx_credential_get_payment_txn: Creating callback")
            Credential.get_payment_txn.cb = create_cb(CFUNCTYPE(None, c_uint32, c_uint32, c_char_p))

        c_credential_handle = c_uint32(self.handle)

        payment_txn = await do_call('vcx_credential_get_payment_txn',
                      c_credential_handle,
                      Credential.get_payment_txn.cb)

        return json.loads(payment_txn.decode())








