from ctypes import *
from vcx.error import VcxError, ErrorCode
from vcx.api.vcx_base import VcxBase
from vcx.common import do_call, create_cb, error_message

import json


class CredentialDef(VcxBase):

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
        try:
            credential_def = await CredentialDef._deserialize("vcx_credentialdef_deserialize",
                                                              json.dumps(data),
                                                              data['source_id'],
                                                              data['name'],
                                                              data['id'])
            return credential_def
        except KeyError:
            raise VcxError(ErrorCode.InvalidCredentialDef, error_message(ErrorCode.InvalidCredentialDef))

    async def serialize(self) -> dict:
        return await self._serialize(CredentialDef, 'vcx_credentialdef_serialize')

    async def get_cred_def_id(self):
        cb = create_cb(CFUNCTYPE(None, c_uint32, c_uint32, c_char_p))
        c_handle = c_uint32(self.handle)
        cred_def_id = await do_call('vcx_credentialdef_get_cred_def_id', c_handle, cb)
        return cred_def_id .decode()

    def release(self) -> None:
        self._release(CredentialDef, 'vcx_credentialdef_release')
