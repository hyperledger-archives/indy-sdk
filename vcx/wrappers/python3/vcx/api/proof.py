from ctypes import *
from vcx.common import do_call, create_cb
from vcx.api.connection import Connection
from vcx.api.vcx_stateful import VcxStateful

import json

class Proof(VcxStateful):

    def __init__(self, source_id: str):
        VcxStateful.__init__(self, source_id)
        self._proof_state = 0

    def __del__(self):
        self.release()
        self.logger.debug("Deleted {} obj: {}".format(Proof, self.handle))

    @property
    def proof_state(self):
        return self._proof_state

    @proof_state.setter
    def proof_state(self, x):
        self._proof_state = x

    @staticmethod
    async def create(source_id: str, name: str, requested_attrs: list, revocation_interval: dict, requested_predicates: list = []):
        """
         Builds a generic proof object
        :param source_id: Tag associated by user of sdk
        :param name: Name of the Proof
        :param requested_attrs: Attributes associated with the Proof
           {
               "name": Optional<string>, // attribute name, (case insensitive and ignore spaces)
               "names": Optional<[string, string]>, // attribute names, (case insensitive and ignore spaces)
                                                    // NOTE: should either be "name" or "names", not both and not none of them.
                                                    // Use "names" to specify several attributes that have to match a single credential.
               "restrictions":  (filter_json) {
                  "schema_id": string, (Optional)
                  "schema_issuer_did": string, (Optional)
                  "schema_name": string, (Optional)
                  "schema_version": string, (Optional)
                  "issuer_did": string, (Optional)
                  "cred_def_id": string, (Optional)
              },
               "non_revoked": {
                   "from": Optional<(u64)> Requested time represented as a total number of seconds from Unix Epoch, Optional
                   "to": Optional<(u64)>
                       //Requested time represented as a total number of seconds from Unix Epoch, Optional
               }
           }
        :param requested_predicates: Predicates associated with the Proof
           { // set of requested predicates
              "name": attribute name, (case insensitive and ignore spaces)
              "p_type": predicate type (Currently ">=" only)
              "p_value": int predicate value
              "restrictions": Optional<filter_json>, // see above
              "non_revoked": Optional<{
                  "from": Optional<(u64)> Requested time represented as a total number of seconds from Unix Epoch, Optional
                  "to": Optional<(u64)> Requested time represented as a total number of seconds from Unix Epoch, Optional
              }>
           }
        :param revocation_interval: interval applied to all requested attributes indicating when the claim must be valid (NOT revoked)
        Example:
        name = "proof name"
        requested_attrs = [{"name": "age", "restrictions": [{"schema_id": "6XFh8yBzrpJQmNyZzgoTqB:2:schema_name:0.0.11", "schema_name":"Faber Student Info", "schema_version":"1.0", "schema_issuer_did":"6XFh8yBzrpJQmNyZzgoTqB", "issuer_did":"8XFh8yBzrpJQmNyZzgoTqB", "cred_def_id": "8XFh8yBzrpJQmNyZzgoTqB:3:CL:1766" }, { "schema_id": "5XFh8yBzrpJQmNyZzgoTqB:2:schema_name:0.0.11", "schema_name":"BYU Student Info", "schema_version":"1.0", "schema_issuer_did":"5XFh8yBzrpJQmNyZzgoTqB", "issuer_did":"66Fh8yBzrpJQmNyZzgoTqB", "cred_def_id": "66Fh8yBzrpJQmNyZzgoTqB:3:CL:1766" } ] }, { "names":["name", "male"], "restrictions": [ { "schema_id": "6XFh8yBzrpJQmNyZzgoTqB:2:schema_name:0.0.11", "schema_name":"Faber Student Info", "schema_version":"1.0", "schema_issuer_did":"6XFh8yBzrpJQmNyZzgoTqB", "issuer_did":"8XFh8yBzrpJQmNyZzgoTqB", "cred_def_id": "8XFh8yBzrpJQmNyZzgoTqB:3:CL:1766" }, { "schema_id": "5XFh8yBzrpJQmNyZzgoTqB:2:schema_name:0.0.11", "schema_name":"BYU Student Info", "schema_version":"1.0", "schema_issuer_did":"5XFh8yBzrpJQmNyZzgoTqB", "issuer_did":"66Fh8yBzrpJQmNyZzgoTqB", "cred_def_id": "66Fh8yBzrpJQmNyZzgoTqB:3:CL:1766"}]}]
        revocation_interval = {"from": 1, "to": 2}  // Both values are optional
        proof = await Proof.create(source_id, name, requested_attrs)
        :return: Proof Object
        """
        constructor_params = (source_id,)

        c_source_id = c_char_p(source_id.encode('utf-8'))
        c_name = c_char_p(name.encode('utf-8'))
        c_req_predicates = c_char_p(json.dumps(requested_predicates).encode('utf-8'))
        c_req_attrs = c_char_p(json.dumps(requested_attrs).encode('utf-8'))
        c_revocation_interval = c_char_p(json.dumps(revocation_interval).encode('utf-8'))
        c_params = (c_source_id, c_req_attrs, c_req_predicates, c_revocation_interval, c_name)

        return await Proof._create("vcx_proof_create",
                                   constructor_params,
                                   c_params)

    @staticmethod
    async def deserialize(data: dict):
        """
        Builds a Proof object with defined attributes.
        Attributes are provided by a previous call to the serialize function.
        :param data:
        Example:
        name = "proof name"
        requested_attrs = [{"name": "age", "restrictions": [{"schema_id": "6XFh8yBzrpJQmNyZzgoTqB:2:schema_name:0.0.11", "schema_name":"Faber Student Info", "schema_version":"1.0", "schema_issuer_did":"6XFh8yBzrpJQmNyZzgoTqB", "issuer_did":"8XFh8yBzrpJQmNyZzgoTqB", "cred_def_id": "8XFh8yBzrpJQmNyZzgoTqB:3:CL:1766" }, { "schema_id": "5XFh8yBzrpJQmNyZzgoTqB:2:schema_name:0.0.11", "schema_name":"BYU Student Info", "schema_version":"1.0", "schema_issuer_did":"5XFh8yBzrpJQmNyZzgoTqB", "issuer_did":"66Fh8yBzrpJQmNyZzgoTqB", "cred_def_id": "66Fh8yBzrpJQmNyZzgoTqB:3:CL:1766" } ] }, { "name":"name", "restrictions": [ { "schema_id": "6XFh8yBzrpJQmNyZzgoTqB:2:schema_name:0.0.11", "schema_name":"Faber Student Info", "schema_version":"1.0", "schema_issuer_did":"6XFh8yBzrpJQmNyZzgoTqB", "issuer_did":"8XFh8yBzrpJQmNyZzgoTqB", "cred_def_id": "8XFh8yBzrpJQmNyZzgoTqB:3:CL:1766" }, { "schema_id": "5XFh8yBzrpJQmNyZzgoTqB:2:schema_name:0.0.11", "schema_name":"BYU Student Info", "schema_version":"1.0", "schema_issuer_did":"5XFh8yBzrpJQmNyZzgoTqB", "issuer_did":"66Fh8yBzrpJQmNyZzgoTqB", "cred_def_id": "66Fh8yBzrpJQmNyZzgoTqB:3:CL:1766"}]}]
        proof = await Proof.create(source_id, name, requested_attrs)
        data = proof.serialize()
        proof2 = await Proof.deserialize(data)
        :return: Proof Object
        """
        return await Proof._deserialize("vcx_proof_deserialize",
                                        json.dumps(data),
                                        data.get('data').get('source_id'))

    async def serialize(self) -> dict:
        """
        Data returned can be used to recreate an entity by passing it to the deserialize function.
        Same json object structure that is passed to the deserialize function.
        Example:
        name = "proof name"
        requested_attrs = [{"name": "age", "restrictions": [{"schema_id": "6XFh8yBzrpJQmNyZzgoTqB:2:schema_name:0.0.11", "schema_name":"Faber Student Info", "schema_version":"1.0", "schema_issuer_did":"6XFh8yBzrpJQmNyZzgoTqB", "issuer_did":"8XFh8yBzrpJQmNyZzgoTqB", "cred_def_id": "8XFh8yBzrpJQmNyZzgoTqB:3:CL:1766" }, { "schema_id": "5XFh8yBzrpJQmNyZzgoTqB:2:schema_name:0.0.11", "schema_name":"BYU Student Info", "schema_version":"1.0", "schema_issuer_did":"5XFh8yBzrpJQmNyZzgoTqB", "issuer_did":"66Fh8yBzrpJQmNyZzgoTqB", "cred_def_id": "66Fh8yBzrpJQmNyZzgoTqB:3:CL:1766" } ] }, { "name":"name", "restrictions": [ { "schema_id": "6XFh8yBzrpJQmNyZzgoTqB:2:schema_name:0.0.11", "schema_name":"Faber Student Info", "schema_version":"1.0", "schema_issuer_did":"6XFh8yBzrpJQmNyZzgoTqB", "issuer_did":"8XFh8yBzrpJQmNyZzgoTqB", "cred_def_id": "8XFh8yBzrpJQmNyZzgoTqB:3:CL:1766" }, { "schema_id": "5XFh8yBzrpJQmNyZzgoTqB:2:schema_name:0.0.11", "schema_name":"BYU Student Info", "schema_version":"1.0", "schema_issuer_did":"5XFh8yBzrpJQmNyZzgoTqB", "issuer_did":"66Fh8yBzrpJQmNyZzgoTqB", "cred_def_id": "66Fh8yBzrpJQmNyZzgoTqB:3:CL:1766"}]}]
        proof = await Proof.create(source_id, name, requested_attrs)
        data = proof.serialize()
        :return: String
        """
        return await self._serialize(Proof, 'vcx_proof_serialize')

    async def update_state(self) -> int:
        """
        Communicates with the agent service for polling and setting the state of the entity.
        Example:
        name = "proof name"
        requested_attrs = [{"name": "age", "restrictions": [{"schema_id": "6XFh8yBzrpJQmNyZzgoTqB:2:schema_name:0.0.11", "schema_name":"Faber Student Info", "schema_version":"1.0", "schema_issuer_did":"6XFh8yBzrpJQmNyZzgoTqB", "issuer_did":"8XFh8yBzrpJQmNyZzgoTqB", "cred_def_id": "8XFh8yBzrpJQmNyZzgoTqB:3:CL:1766" }, { "schema_id": "5XFh8yBzrpJQmNyZzgoTqB:2:schema_name:0.0.11", "schema_name":"BYU Student Info", "schema_version":"1.0", "schema_issuer_did":"5XFh8yBzrpJQmNyZzgoTqB", "issuer_did":"66Fh8yBzrpJQmNyZzgoTqB", "cred_def_id": "66Fh8yBzrpJQmNyZzgoTqB:3:CL:1766" } ] }, { "name":"name", "restrictions": [ { "schema_id": "6XFh8yBzrpJQmNyZzgoTqB:2:schema_name:0.0.11", "schema_name":"Faber Student Info", "schema_version":"1.0", "schema_issuer_did":"6XFh8yBzrpJQmNyZzgoTqB", "issuer_did":"8XFh8yBzrpJQmNyZzgoTqB", "cred_def_id": "8XFh8yBzrpJQmNyZzgoTqB:3:CL:1766" }, { "schema_id": "5XFh8yBzrpJQmNyZzgoTqB:2:schema_name:0.0.11", "schema_name":"BYU Student Info", "schema_version":"1.0", "schema_issuer_did":"5XFh8yBzrpJQmNyZzgoTqB", "issuer_did":"66Fh8yBzrpJQmNyZzgoTqB", "cred_def_id": "66Fh8yBzrpJQmNyZzgoTqB:3:CL:1766"}]}]
        proof = await Proof.create(source_id, name, requested_attrs)
        assert await proof.update_state() == State.Initialized
        :return: StateType
        """
        return await self._update_state(Proof, 'vcx_proof_update_state')

    async def update_state_with_message(self, message: str) -> int:
        """
        Update the state of the proof based on the given message.
        Example:
        proof = await Proof.create(source_id)
        assert await proof.update_state_with_message(message) == State.Accepted
        :param message:
        :return Current state of the Proof
        """
        return await self._update_state_with_message(Connection, message, 'vcx_proof_update_state_with_message')

    async def get_state(self) -> int:
        """
        Gets the state of the entity.
        Example:
        name = "proof name"
        requested_attrs = [{"name": "age", "restrictions": [{"schema_id": "6XFh8yBzrpJQmNyZzgoTqB:2:schema_name:0.0.11", "schema_name":"Faber Student Info", "schema_version":"1.0", "schema_issuer_did":"6XFh8yBzrpJQmNyZzgoTqB", "issuer_did":"8XFh8yBzrpJQmNyZzgoTqB", "cred_def_id": "8XFh8yBzrpJQmNyZzgoTqB:3:CL:1766" }, { "schema_id": "5XFh8yBzrpJQmNyZzgoTqB:2:schema_name:0.0.11", "schema_name":"BYU Student Info", "schema_version":"1.0", "schema_issuer_did":"5XFh8yBzrpJQmNyZzgoTqB", "issuer_did":"66Fh8yBzrpJQmNyZzgoTqB", "cred_def_id": "66Fh8yBzrpJQmNyZzgoTqB:3:CL:1766" } ] }, { "name":"name", "restrictions": [ { "schema_id": "6XFh8yBzrpJQmNyZzgoTqB:2:schema_name:0.0.11", "schema_name":"Faber Student Info", "schema_version":"1.0", "schema_issuer_did":"6XFh8yBzrpJQmNyZzgoTqB", "issuer_did":"8XFh8yBzrpJQmNyZzgoTqB", "cred_def_id": "8XFh8yBzrpJQmNyZzgoTqB:3:CL:1766" }, { "schema_id": "5XFh8yBzrpJQmNyZzgoTqB:2:schema_name:0.0.11", "schema_name":"BYU Student Info", "schema_version":"1.0", "schema_issuer_did":"5XFh8yBzrpJQmNyZzgoTqB", "issuer_did":"66Fh8yBzrpJQmNyZzgoTqB", "cred_def_id": "66Fh8yBzrpJQmNyZzgoTqB:3:CL:1766"}]}]
        proof = await Proof.create(source_id, name, requested_attrs)
        assert await proof.get_state() == State.Initialized
        :return: StateType
        """
        return await self._get_state(Proof, 'vcx_proof_get_state')

    def release(self) -> None:
        """
        Internal method used for memory management
        :return: None
        """
        self._release(Proof, 'vcx_proof_release')

    async def get_proof_request_msg(self):
        """
        Example:
        name = "proof name"
        requested_attrs = [{"name": "age", "restrictions": [{"schema_id": "6XFh8yBzrpJQmNyZzgoTqB:2:schema_name:0.0.11", "schema_name":"Faber Student Info", "schema_version":"1.0", "schema_issuer_did":"6XFh8yBzrpJQmNyZzgoTqB", "issuer_did":"8XFh8yBzrpJQmNyZzgoTqB", "cred_def_id": "8XFh8yBzrpJQmNyZzgoTqB:3:CL:1766" }, { "schema_id": "5XFh8yBzrpJQmNyZzgoTqB:2:schema_name:0.0.11", "schema_name":"BYU Student Info", "schema_version":"1.0", "schema_issuer_did":"5XFh8yBzrpJQmNyZzgoTqB", "issuer_did":"66Fh8yBzrpJQmNyZzgoTqB", "cred_def_id": "66Fh8yBzrpJQmNyZzgoTqB:3:CL:1766" } ] }, { "name":"name", "restrictions": [ { "schema_id": "6XFh8yBzrpJQmNyZzgoTqB:2:schema_name:0.0.11", "schema_name":"Faber Student Info", "schema_version":"1.0", "schema_issuer_did":"6XFh8yBzrpJQmNyZzgoTqB", "issuer_did":"8XFh8yBzrpJQmNyZzgoTqB", "cred_def_id": "8XFh8yBzrpJQmNyZzgoTqB:3:CL:1766" }, { "schema_id": "5XFh8yBzrpJQmNyZzgoTqB:2:schema_name:0.0.11", "schema_name":"BYU Student Info", "schema_version":"1.0", "schema_issuer_did":"5XFh8yBzrpJQmNyZzgoTqB", "issuer_did":"66Fh8yBzrpJQmNyZzgoTqB", "cred_def_id": "66Fh8yBzrpJQmNyZzgoTqB:3:CL:1766"}]}]
        proof = await Proof.create(source_id, name, requested_attrs)
        await proof.get_proof_request_msg()
        :param
        :return:
        """
        if not hasattr(Proof.get_proof_request_msg, "cb"):
            self.logger.debug("vcx_proof_send_request: Creating callback")
            Proof.get_proof_request_msg.cb = create_cb(CFUNCTYPE(None, c_uint32, c_uint32, c_char_p))

        c_proof_handle = c_uint32(self.handle)

        msg = await do_call('vcx_proof_get_request_msg',
                      c_proof_handle,
                      Proof.get_proof_request_msg.cb)

        return json.loads(msg.decode())

    async def request_proof(self, connection: Connection):
        """
        Example:
        connection = await Connection.create(source_id)
        await connection.connect(phone_number)
        name = "proof name"
        requested_attrs = [{"name": "age", "restrictions": [{"schema_id": "6XFh8yBzrpJQmNyZzgoTqB:2:schema_name:0.0.11", "schema_name":"Faber Student Info", "schema_version":"1.0", "schema_issuer_did":"6XFh8yBzrpJQmNyZzgoTqB", "issuer_did":"8XFh8yBzrpJQmNyZzgoTqB", "cred_def_id": "8XFh8yBzrpJQmNyZzgoTqB:3:CL:1766" }, { "schema_id": "5XFh8yBzrpJQmNyZzgoTqB:2:schema_name:0.0.11", "schema_name":"BYU Student Info", "schema_version":"1.0", "schema_issuer_did":"5XFh8yBzrpJQmNyZzgoTqB", "issuer_did":"66Fh8yBzrpJQmNyZzgoTqB", "cred_def_id": "66Fh8yBzrpJQmNyZzgoTqB:3:CL:1766" } ] }, { "name":"name", "restrictions": [ { "schema_id": "6XFh8yBzrpJQmNyZzgoTqB:2:schema_name:0.0.11", "schema_name":"Faber Student Info", "schema_version":"1.0", "schema_issuer_did":"6XFh8yBzrpJQmNyZzgoTqB", "issuer_did":"8XFh8yBzrpJQmNyZzgoTqB", "cred_def_id": "8XFh8yBzrpJQmNyZzgoTqB:3:CL:1766" }, { "schema_id": "5XFh8yBzrpJQmNyZzgoTqB:2:schema_name:0.0.11", "schema_name":"BYU Student Info", "schema_version":"1.0", "schema_issuer_did":"5XFh8yBzrpJQmNyZzgoTqB", "issuer_did":"66Fh8yBzrpJQmNyZzgoTqB", "cred_def_id": "66Fh8yBzrpJQmNyZzgoTqB:3:CL:1766"}]}]
        proof = await Proof.create(source_id, name, requested_attrs)
        await proof.request_proof(connection)
        :param connection: Connection
        :return:
        """
        if not hasattr(Proof.request_proof, "cb"):
            self.logger.debug("vcx_proof_send_request: Creating callback")
            Proof.request_proof.cb = create_cb(CFUNCTYPE(None, c_uint32, c_uint32))

        c_proof_handle = c_uint32(self.handle)
        c_connection_handle = c_uint32(connection.handle)

        await do_call('vcx_proof_send_request',
                      c_proof_handle,
                      c_connection_handle,
                      Proof.request_proof.cb)

    async def get_proof(self, connection: Connection) -> list:
        """
        Example:
        connection = await Connection.create(source_id)
        await connection.connect(phone_number)
        name = "proof name"
        requested_attrs = [{"name": "age", "restrictions": [{"schema_id": "6XFh8yBzrpJQmNyZzgoTqB:2:schema_name:0.0.11", "schema_name":"Faber Student Info", "schema_version":"1.0", "schema_issuer_did":"6XFh8yBzrpJQmNyZzgoTqB", "issuer_did":"8XFh8yBzrpJQmNyZzgoTqB", "cred_def_id": "8XFh8yBzrpJQmNyZzgoTqB:3:CL:1766" }, { "schema_id": "5XFh8yBzrpJQmNyZzgoTqB:2:schema_name:0.0.11", "schema_name":"BYU Student Info", "schema_version":"1.0", "schema_issuer_did":"5XFh8yBzrpJQmNyZzgoTqB", "issuer_did":"66Fh8yBzrpJQmNyZzgoTqB", "cred_def_id": "66Fh8yBzrpJQmNyZzgoTqB:3:CL:1766" } ] }, { "name":"name", "restrictions": [ { "schema_id": "6XFh8yBzrpJQmNyZzgoTqB:2:schema_name:0.0.11", "schema_name":"Faber Student Info", "schema_version":"1.0", "schema_issuer_did":"6XFh8yBzrpJQmNyZzgoTqB", "issuer_did":"8XFh8yBzrpJQmNyZzgoTqB", "cred_def_id": "8XFh8yBzrpJQmNyZzgoTqB:3:CL:1766" }, { "schema_id": "5XFh8yBzrpJQmNyZzgoTqB:2:schema_name:0.0.11", "schema_name":"BYU Student Info", "schema_version":"1.0", "schema_issuer_did":"5XFh8yBzrpJQmNyZzgoTqB", "issuer_did":"66Fh8yBzrpJQmNyZzgoTqB", "cred_def_id": "66Fh8yBzrpJQmNyZzgoTqB:3:CL:1766"}]}]
        proof = await Proof.create(source_id, name, requested_attrs)
        await proof.request_proof(connection)
        await proof.get_proof(connection)
        :param connection: Handle for the connection to receive a proof from.
        :return: List of proofs received from the given connection.
        """
        if not hasattr(Proof.get_proof, "cb"):
            self.logger.debug("vcx_get_proof: Creating callback")
            Proof.get_proof.cb = create_cb(CFUNCTYPE(None, c_uint32, c_uint32, c_uint32, c_char_p))

        c_proof_handle = c_uint32(self.handle)
        c_connection_handle = c_uint32(connection.handle)

        proof_state, proof = await do_call('vcx_get_proof',
                                           c_proof_handle,
                                           c_connection_handle,
                                           Proof.get_proof.cb)
        self.proof_state = proof_state
        return json.loads(proof.decode())
