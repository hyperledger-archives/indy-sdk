from typing import Optional
from ctypes import *
from vcx.common import do_call, create_cb
from vcx.api.connection import Connection
from vcx.api.vcx_stateful import VcxStateful

import json


class DisclosedProof(VcxStateful):

    def __init__(self, source_id: str):
        VcxStateful.__init__(self, source_id)
        self._name = source_id
        self._proof_req = None

    def __del__(self):
        self.release()
        self.logger.debug("Deleted {} obj: {}".format(DisclosedProof, self.handle))

    @property
    def proof_request(self):
        return self._proof_req

    @proof_request.setter
    def proof_request(self, x):
        self._proof_req = x

    @staticmethod
    async def create(source_id: str, proof_request: str):
        """
        Create a proof for fulfilling a corresponding proof request
        :param source_id: Tag associated by user of sdk
        :param proof_request: Proof Request data sent by requestor.
        Example:
        source_id = 'sourceId'
        request = {
            "@topic": {
            "mid": 9,
            "tid": 1
            },
            "@type": {
                "name": "PROOF_REQUEST",
                "version":"1.0"
            },
            "msg_ref_id": "ymy5nth",
            "proof_request_data": {
                "name": "Account Certificate",
                "nonce": "838186471541979035208225",
                "requested_attributes": {
                    "business_2": {
                        "name": "business"
                    },
                    "email_1": {
                        "name": "email"
                    },
                    "name_0": {
                        "name": "name"
                    }
                },
                "requested_predicates": {},
                "version": "0.1"
            }
        }
        disclosed_proof = await DisclosedProof.create(source_id, request)
        :return: Disclosed Proof Object
        """
        constructor_params = (source_id,)

        c_source_id = c_char_p(source_id.encode('utf-8'))
        c_proof_request = c_char_p(json.dumps(proof_request).encode('utf-8'))
        c_params = (c_source_id, c_proof_request, )

        return await DisclosedProof._create("vcx_disclosed_proof_create_with_request",
                                   constructor_params,
                                   c_params)

    @staticmethod
    async def create_with_msgid(source_id: str, connection: Connection, msg_id: str):
        """

        :param source_id:
        :param connection:
        :param msg_id:
        Example:
        msg_id = '1'
        phone_number = '8019119191'
        connection = await Connection.create(source_id)
        await connection.connect(phone_number)
        disclosed_proof = await DisclosedProof.create_with_msgid(source_id, connection, msg_id)
        :return: DisclosedProof
        """
        proof = DisclosedProof(source_id)

        c_source_id = c_char_p(source_id.encode('utf-8'))
        c_msg_id = c_char_p(json.dumps(msg_id).encode('utf-8'))
        c_connection_handle = c_uint32(connection.handle)

        if not hasattr(DisclosedProof.create_with_msgid, "cb"):
            DisclosedProof.create_with_msgid.cb = create_cb(CFUNCTYPE(None, c_uint32, c_uint32, c_uint32, c_char_p))

        proof.handle, proof_req = await do_call('vcx_disclosed_proof_create_with_msgid',
                                                c_source_id,
                                                c_connection_handle,
                                                c_msg_id,
                                                DisclosedProof.create_with_msgid.cb)

        proof.proof_request = json.loads(proof_req.decode())

        return proof

    @staticmethod
    async def deserialize(data: dict):
        """
        :param data: Data provided by the serialize method
        Example:
        msg_id = '1'
        phone_number = '8019119191'
        connection = await Connection.create(source_id)
        await connection.connect(phone_number)
        disclosed_proof = await DisclosedProof.create_with_msgid(source_id, connection, msg_id)
        data = await disclosed_proof.serialize()
        disclosed_proof2 = await DisclosedProof.deserialize(data)
        :return:  DisclosedProof
        """
        disclosed_proof = await DisclosedProof._deserialize("vcx_disclosed_proof_deserialize",
                                                      json.dumps(data),
                                                      data.get('data').get('source_id'))
        return disclosed_proof

    @staticmethod
    async def get_requests(connection: Connection) -> dict:
        """
        Example:
        msg_id = '1'
        phone_number = '8019119191'
        connection = await Connection.create(source_id)
        await connection.connect(phone_number)
        disclosed_proof = await DisclosedProof.create_with_msgid(source_id, connection, msg_id)
        requests = await DisclosedProof.get_requests(connection)
        :param connection: Connection
        :return: requests associated with the connection
        """
        if not hasattr(DisclosedProof.get_requests, "cb"):
            DisclosedProof.get_requests.cb = create_cb(CFUNCTYPE(None, c_uint32, c_uint32, c_char_p))

        c_connection_handle = c_uint32(connection.handle)

        data = await do_call('vcx_disclosed_proof_get_requests',
                      c_connection_handle,
                      DisclosedProof.get_requests.cb)

        return json.loads(data.decode())

    async def serialize(self) -> dict:
        """
        Serialize the object
        Example:
        msg_id = '1'
        phone_number = '8019119191'
        connection = await Connection.create(source_id)
        await connection.connect(phone_number)
        disclosed_proof = await DisclosedProof.create_with_msgid(source_id, connection, msg_id)
        data = await disclosed_proof.serialize()
        :return:
        """
        return await self._serialize(DisclosedProof, 'vcx_disclosed_proof_serialize')

    async def update_state(self) -> int:
        """
        Example:
        msg_id = '1'
        phone_number = '8019119191'
        connection = await Connection.create(source_id)
        await connection.connect(phone_number)
        disclosed_proof = await DisclosedProof.create_with_msgid(source_id, connection, msg_id)
        assert await disclosed_proof.update_state() == State.RequestReceived
        :return:
        """
        return await self._update_state(DisclosedProof, 'vcx_disclosed_proof_update_state')

    async def update_state_with_message(self, message: str) -> int:
        """
        Example:
        msg_id = '1'
        phone_number = '8019119191'
        connection = await Connection.create(source_id)
        await connection.connect(phone_number)
        disclosed_proof = await DisclosedProof.create_with_msgid(source_id, connection, msg_id)
        assert await disclosed_proof.update_state_with_message(msg) == State.RequestReceived
        :return:
        """
        return await self._update_state_with_message(DisclosedProof, message, 'vcx_disclosed_proof_update_state_with_message')

    async def get_state(self) -> int:
        """

        Gets the state of the entity.
        Example:
        msg_id = '1'
        phone_number = '8019119191'
        connection = await Connection.create(source_id)
        await connection.connect(phone_number)
        disclosed_proof = await DisclosedProof.create_with_msgid(source_id, connection, msg_id)
        assert await proof.get_state() == State.Initialized
        :return: StateType
        """
        return await self._get_state(DisclosedProof, 'vcx_disclosed_proof_get_state')

    def release(self) -> None:
        """
        Internal method used for memory management
        :return: None
        """
        self._release(DisclosedProof, 'vcx_disclosed_proof_release')

    async def get_creds(self) -> dict:
        """
        Gets the credentials from a disclosed proof
        Example:
        msg_id = '1'
        phone_number = '8019119191'
        connection = await Connection.create(source_id)
        await connection.connect(phone_number)
        disclosed_proof = await DisclosedProof.create_with_msgid(source_id, connection, msg_id)
        creds = await disclosed_proof.get_creds()
        :return: credentials
        """
        if not hasattr(DisclosedProof.get_creds, "cb"):
            self.logger.debug("vcx_disclosed_proof_retrieve_credentials: Creating callback")
            DisclosedProof.get_creds.cb = create_cb(CFUNCTYPE(None, c_uint32, c_uint32, c_char_p))

        c_disclosed_proof_handle = c_uint32(self.handle)

        data = await do_call('vcx_disclosed_proof_retrieve_credentials',
                             c_disclosed_proof_handle,
                             DisclosedProof.get_creds.cb)
        return json.loads(data.decode())

    async def send_proof(self, connection: Optional[Connection] = None):
        """
        Sends the proof to the Connection
        Example:
        msg_id = '1'
        phone_number = '8019119191'
        connection = await Connection.create(source_id)
        await connection.connect(phone_number)
        disclosed_proof = await DisclosedProof.create_with_msgid(source_id, connection, msg_id)
        await disclosed_proof.send_proof(connection)
        :param connection: Connection
        :return: None
        """
        if not hasattr(DisclosedProof.send_proof, "cb"):
            self.logger.debug("vcx_disclosed_proof_send_proof: Creating callback")
            DisclosedProof.send_proof.cb = create_cb(CFUNCTYPE(None, c_uint32, c_uint32))

        c_disclosed_proof_handle = c_uint32(self.handle)
        c_connection_handle = c_uint32(connection.handle) if connection else 0

        await do_call('vcx_disclosed_proof_send_proof',
                      c_disclosed_proof_handle,
                      c_connection_handle,
                      DisclosedProof.send_proof.cb)

    async def get_send_proof_msg(self):
        """
        Example:
        msg = await disclosed_proof.get_send_proof_msg()
        :param
        :return:
        """
        if not hasattr(DisclosedProof.get_send_proof_msg, "cb"):
            self.logger.debug("vcx_proof_send_request: Creating callback")
            DisclosedProof.get_send_proof_msg.cb = create_cb(CFUNCTYPE(None, c_uint32, c_uint32, c_char_p))

        c_proof_handle = c_uint32(self.handle)

        msg = await do_call('vcx_disclosed_proof_get_proof_msg',
                      c_proof_handle,
                      DisclosedProof.get_send_proof_msg.cb)

        return json.loads(msg.decode())


    async def generate_proof(self, selected_creds: dict, self_attested_attrs: dict):
        """
        Generates the proof
        Example:
        msg_id = '1'
        phone_number = '8019119191'
        connection = await Connection.create(source_id)
        await connection.connect(phone_number)
        disclosed_proof = await DisclosedProof.create_with_msgid(source_id, connection, msg_id)
        await disclosed_proof.generate_proof({}, {})
        :param selected_creds: Credentials issued
        :param self_attested_attrs: Self Attested Attributes
        :return: None
        """
        if not hasattr(DisclosedProof.generate_proof, "cb"):
            self.logger.debug("vcx_disclosed_proof_generate_proof: Creating callback")
            DisclosedProof.generate_proof.cb = create_cb(CFUNCTYPE(None, c_uint32, c_uint32))

        c_disclosed_proof_handle = c_uint32(self.handle)
        c_selected_creds = c_char_p(json.dumps(selected_creds).encode('utf-8'))
        c_self_attested_attrs = c_char_p(json.dumps(self_attested_attrs).encode('utf-8'))

        await do_call('vcx_disclosed_proof_generate_proof',
                      c_disclosed_proof_handle,
                      c_selected_creds,
                      c_self_attested_attrs,
                      DisclosedProof.generate_proof.cb)


    async def decline_presentation_request(self, connection: Connection,
                                           reason: Optional[str] = None, proposal: Optional[dict] = None):
        """
        Declines presentation request.
        There are two ways of following interaction:
            - Prover wants to propose using a different presentation - pass `proposal` parameter
            - Prover doesn't want to continue interaction - pass `reason` parameter.
        Note that only one of these parameters can be passed.

        Note that this function is useful in case `aries` communication method is used.
        In other cases it returns ActionNotSupported error.

        :param connection: Connection
        :param reason: human-readable string that explain the reason of decline
        :param proposal: the proposed format of presentation request
           (see https://github.com/hyperledger/aries-rfcs/tree/master/features/0037-present-proof#presentation-preview for details)
           {
              "attributes": [
                  {
                      "name": "<attribute_name>",
                      "cred_def_id": Optional("<cred_def_id>"),
                      "mime-type": Optional("<type>"),
                      "value": Optional("<value>")
                  },
                  // more attributes
              ],
              "predicates": [
                  {
                      "name": "<attribute_name>",
                      "cred_def_id": Optional("<cred_def_id>"),
                      "predicate": "<predicate>", - one of "<", "<=", ">=", ">"
                      "threshold": <threshold>
                  },
                  // more predicates
              ]
           }

        Example:
        msg_id = '1'
        phone_number = '8019119191'
        connection = await Connection.create(source_id)
        await connection.connect(phone_number)
        disclosed_proof = await DisclosedProof.create_with_msgid(source_id, connection, msg_id)
        await disclosed_proof.decline_presentation_request(connection, 'reason', None)
        :return: None
        """
        if not hasattr(DisclosedProof.decline_presentation_request, "cb"):
            self.logger.debug("vcx_disclosed_proof_decline_presentation_request: Creating callback")
            DisclosedProof.decline_presentation_request.cb = create_cb(CFUNCTYPE(None, c_uint32, c_uint32))

        c_disclosed_proof_handle = c_uint32(self.handle)
        c_connection_handle = c_uint32(connection.handle)
        c_reason = c_char_p(reason.encode('utf-8')) if reason else None
        c_proposal = c_char_p(json.dumps(proposal).encode('utf-8')) if proposal else None

        await do_call('vcx_disclosed_proof_decline_presentation_request',
                      c_disclosed_proof_handle,
                      c_connection_handle,
                      c_reason,
                      c_proposal,
                      DisclosedProof.decline_presentation_request.cb)
