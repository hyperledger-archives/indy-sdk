from ctypes import *
from vcx.error import VcxError, ErrorCode
from vcx.api.vcx_base import VcxBase
from vcx.common import do_call, create_cb, error_message

import json


class CredentialDef(VcxBase):
    """
    Object that represents a credential definition written on the ledger.

    Attributes:
        source_id: user generated unique identifier
        schema_id: the ledger ID of the schema
        name: name of the credential definition
    """

    def __init__(self, source_id: str, name: str, schema_id: str):
        VcxBase.__init__(self, source_id)
        self._source_id = source_id
        self._schema_id = schema_id
        self._name = name

    def __del__(self):
        self.release()
        self.logger.debug("Deleted {} obj: {}".format(CredentialDef, self.handle))

    @property
    def name(self):
        return self._name

    @name.setter
    def name(self, x):
        self._name = x

    @property
    def schema_id(self):
        return self._schema_id

    @schema_id.setter
    def schema_id(self, x):
        self._schema_id = x

    @staticmethod
    async def create(source_id: str, name: str, schema_id: str, payment_handle: int):
        """
        Creates a new CredentialDef object that is written to the ledger

        :param source_id: Institution's unique ID for the credential definition
        :param name: Name of credential definition
        :param schema_id: The schema ID given during the creation of the schema
        :param payment_handle: NYI - payment of ledger fee is taken from wallet automatically
        :return: credential_def object, written to ledger
        """
        constructor_params = (source_id, name, schema_id)

        c_source_id = c_char_p(source_id.encode('utf-8'))
        c_schema_id = c_char_p(schema_id.encode('utf-8'))
        c_name = c_char_p(name.encode('utf-8'))
        # default institution_did in config is used as issuer_did
        c_issuer_did = None
        c_payment = c_uint32(payment_handle)
        # Todo: add params for tag and config
        c_tag = c_char_p('tag1'.encode('utf-8'))
        c_config = c_char_p('{"support_revocation":false}'.encode('utf-8'))
        c_params = (c_source_id, c_name, c_schema_id, c_issuer_did, c_tag, c_config, c_payment)

        return await CredentialDef._create("vcx_credentialdef_create",
                                           constructor_params,
                                           c_params)

    @staticmethod
    async def deserialize(data: dict):
        """
        Create the object from a previously serialized object.

        :param data: The output of the "serialize" call
        :return: A re-instantiated object
        """
        try:
            credential_def = await CredentialDef._deserialize("vcx_credentialdef_deserialize",
                                                              json.dumps(data),
                                                              data['data']['source_id'],
                                                              data['data']['name'],
                                                              data['data']['id'])
            return credential_def
        except KeyError:
            raise VcxError(ErrorCode.InvalidCredentialDef, error_message(ErrorCode.InvalidCredentialDef))

    async def serialize(self) -> dict:
        """
        Serialize the object for storage

        :return: serialized object
        """
        return await self._serialize(CredentialDef, 'vcx_credentialdef_serialize')

    async def get_cred_def_id(self):
        """
        Get the ledger ID of the object

        :return: ID string
        """
        cb = create_cb(CFUNCTYPE(None, c_uint32, c_uint32, c_char_p))
        c_handle = c_uint32(self.handle)
        cred_def_id = await do_call('vcx_credentialdef_get_cred_def_id', c_handle, cb)
        return cred_def_id .decode()

    def release(self) -> None:
        """
        destroy the object and release any memory associated with it

        :return: None
        """
        self._release(CredentialDef, 'vcx_credentialdef_release')

    async def get_payment_txn(self):
        """
        Get the payment transaction information generated when paying the ledger fee

        :return: JSON object with input address and output UTXOs
        """
        if not hasattr(CredentialDef.get_payment_txn, "cb"):
            self.logger.debug("vcx_credentialdef_get_payment_txn: Creating callback")
            CredentialDef.get_payment_txn.cb = create_cb(CFUNCTYPE(None, c_uint32, c_uint32, c_char_p))

        c_credential_handle = c_uint32(self.handle)

        payment_txn = await do_call('vcx_credentialdef_get_payment_txn',
                      c_credential_handle,
                      CredentialDef.get_payment_txn.cb)

        return json.loads(payment_txn.decode())
