from ctypes import *
from vcx.common import do_call, create_cb
from vcx.api.connection import Connection
from vcx.api.vcx_stateful import VcxStateful

import json

class IssuerCredential(VcxStateful):
    """
    The object of the VCX API representing an Issuer side in the credential issuance process.
    Assumes that pairwise connection between Issuer and Holder is already established.

    # State

    The set of object states and transitions depends on communication method is used.
    The communication method can be specified as config option on one of *_init function. The default communication method us `proprietary`.

    proprietary:
        VcxStateType::VcxStateInitialized - once `vcx_issuer_create_credential` (create IssuerCredential object) is called.

        VcxStateType::VcxStateOfferSent - once `vcx_issuer_send_credential_offer` (send `CRED_OFFER` message) is called.

        VcxStateType::VcxStateRequestReceived - once `CRED_REQ` messages is received.
                                                use `vcx_issuer_credential_update_state` or `vcx_issuer_credential_update_state_with_message` functions for state updates.
        VcxStateType::VcxStateAccepted - once `vcx_issuer_send_credential` (send `CRED` message) is called.

    aries:
        VcxStateType::VcxStateInitialized - once `vcx_issuer_create_credential` (create IssuerCredential object) is called.

        VcxStateType::VcxStateOfferSent - once `vcx_issuer_send_credential_offer` (send `CredentialOffer` message) is called.

        VcxStateType::VcxStateRequestReceived - once `CredentialRequest` messages is received.
        VcxStateType::None - once `ProblemReport` messages is received.
                                                use `vcx_issuer_credential_update_state` or `vcx_issuer_credential_update_state_with_message` functions for state updates.

        VcxStateType::VcxStateAccepted - once `vcx_issuer_send_credential` (send `Credential` message) is called.

    # Transitions

    proprietary:
        VcxStateType::None - `vcx_issuer_create_credential` - VcxStateType::VcxStateInitialized

        VcxStateType::VcxStateInitialized - `vcx_issuer_send_credential_offer` - VcxStateType::VcxStateOfferSent

        VcxStateType::VcxStateOfferSent - received `CRED_REQ` - VcxStateType::VcxStateRequestReceived

        VcxStateType::VcxStateRequestReceived - `vcx_issuer_send_credential` - VcxStateType::VcxStateAccepted

    aries: RFC - https://github.com/hyperledger/aries-rfcs/tree/7b6b93acbaf9611d3c892c4bada142fe2613de6e/features/0036-issue-credential
        VcxStateType::None - `vcx_issuer_create_credential` - VcxStateType::VcxStateInitialized

        VcxStateType::VcxStateInitialized - `vcx_issuer_send_credential_offer` - VcxStateType::VcxStateOfferSent

        VcxStateType::VcxStateOfferSent - received `CredentialRequest` - VcxStateType::VcxStateRequestReceived
        VcxStateType::VcxStateOfferSent - received `ProblemReport` - VcxStateType::None

        VcxStateType::VcxStateRequestReceived - vcx_issuer_send_credential` - VcxStateType::VcxStateAccepted

        VcxStateType::VcxStateAccepted - received `Ack` - VcxStateType::VcxStateAccepted

    # Messages

    proprietary:
        CredentialOffer (`CRED_OFFER`)
        CredentialRequest (`CRED_REQ`)
        Credential (`CRED`)

    aries:
        CredentialProposal - https://github.com/hyperledger/aries-rfcs/tree/7b6b93acbaf9611d3c892c4bada142fe2613de6e/features/0036-issue-credential#propose-credential
        CredentialOffer - https://github.com/hyperledger/aries-rfcs/tree/7b6b93acbaf9611d3c892c4bada142fe2613de6e/features/0036-issue-credential#offer-credential
        CredentialRequest - https://github.com/hyperledger/aries-rfcs/tree/7b6b93acbaf9611d3c892c4bada142fe2613de6e/features/0036-issue-credential#request-credential
        Credential - https://github.com/hyperledger/aries-rfcs/tree/7b6b93acbaf9611d3c892c4bada142fe2613de6e/features/0036-issue-credential#issue-credential
        ProblemReport - https://github.com/hyperledger/aries-rfcs/tree/7b6b93acbaf9611d3c892c4bada142fe2613de6e/features/0035-report-problem#the-problem-report-message-type
        Ack - https://github.com/hyperledger/aries-rfcs/tree/master/features/0015-acks#explicit-acks
    """

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
            Create a Issuer Credential object that provides a credential for an enterprise's user
            Assumes a credential definition has been already written to the ledger.

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
            Create a IssuerCredential object from a previously serialized object
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
            Serializes the  issuer credential object for storage and later deserialization.

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
        Query the agency for the received messages.
        Checks for any messages changing state in the object and updates the state attribute.

        Example:
        issuer_credential = await IssuerCredential.create(source_id, attrs, cred_def_id, name, price)
        issuer_credential.update_state()
        :return:
        """
        return await self._update_state(IssuerCredential, 'vcx_issuer_credential_update_state')

    async def update_state_with_message(self, message: str) -> int:
        """
        Update the state of the credential based on the given message.
        Example:
        cred = await IssuerCredential.create(source_id)
        assert await cred.update_state_with_message(message) == State.Accepted
        :param message: message to process for state changes
        :return Current state of the IssuerCredential
        """
        return await self._update_state_with_message(IssuerCredential, message, 'vcx_issuer_credential_update_state_with_message')

    async def get_state(self) -> int:
        """
        Get the current state of the issuer credential object
        Example:
        issuer_credential = await IssuerCredential.create(source_id, attrs, cred_def_id, name, price)
        issuer_credential.update_state()
        :return: State of the Object. Possible states:
                                         1 - Initialized
                                         2 - Offer Sent
                                         3 - Request Received
                                         4 - Issued
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
        Send a credential offer to a holder showing what will be included in the actual credential
        :param connection: Connection that identifies pairwise connection
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

    async def get_offer_msg(self):
        """
        Gets the offer message that can be sent to the specified connection
        :param connection: Connection that identifies pairwise connection
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
        issuer_credential.get_offer_msg(connection)
        """
        if not hasattr(IssuerCredential.get_offer_msg, "cb"):
            self.logger.debug("vcx_issuer_get_credential_offer_msg: Creating callback")
            IssuerCredential.get_offer_msg.cb = create_cb(CFUNCTYPE(None, c_uint32, c_uint32, c_char_p))

        c_credential_handle = c_uint32(self.handle)

        msg = await do_call('vcx_issuer_get_credential_offer_msg',
                            c_credential_handle,
                            IssuerCredential.get_offer_msg.cb)

        return json.loads(msg.decode())

    async def send_credential(self, connection: Connection):
        """
        Sends the credential to the end user (holder).
        :param connection: Connection that identifies pairwise connection
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

    async def get_credential_msg(self, my_pw_did: str):
        """
        Get the credential to send to the end user (prover).
        :param my_pw_did: my pw did associated with person I'm sending credential to
        :return: None
            Example:
            credential.send_credential(connection)
        """
        if not hasattr(IssuerCredential.get_credential_msg, "cb"):
            self.logger.debug("vcx_issuer_get_credential_msg: Creating callback")
            IssuerCredential.get_credential_msg.cb = create_cb(CFUNCTYPE(None, c_uint32, c_uint32, c_char_p))

        c_credential_handle = c_uint32(self.handle)
        c_my_pw_did = c_char_p(json.dumps(my_pw_did).encode('utf-8'))

        msg = await do_call('vcx_issuer_get_credential_msg',
                            c_credential_handle,
                            c_my_pw_did,
                            IssuerCredential.get_credential_msg.cb)

        return json.loads(msg.decode())

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
        Retrieve the payment transaction associated with this credential. This can be used to get the txn that
        was used to pay the issuer from the prover.  This could be considered a receipt of payment from the payer to
        the issuer.

        Example:
        txn = credential.get_payment_txn()
        :return: payment transaction
          {
              "amount":25,
              "inputs":[
                  "pay:null:1_3FvPC7dzFbQKzfG",
                  "pay:null:1_lWVGKc07Pyc40m6"
              ],
              "outputs":[
                  {"recipient":"pay:null:FrSVC3IrirScyRh","amount":5,"extra":null},
                  {"recipient":"pov:null:OsdjtGKavZDBuG2xFw2QunVwwGs5IB3j","amount":25,"extra":null}
              ]
          }
        """
        if not hasattr(IssuerCredential.get_payment_txn, "cb"):
            self.logger.debug("vcx_issuer_credential_get_payment_txn: Creating callback")
            IssuerCredential.get_payment_txn.cb = create_cb(CFUNCTYPE(None, c_uint32, c_uint32, c_char_p))

        c_credential_handle = c_uint32(self.handle)

        payment_txn = await do_call('vcx_issuer_credential_get_payment_txn',
                      c_credential_handle,
                      IssuerCredential.get_payment_txn.cb)

        return json.loads(payment_txn.decode())

