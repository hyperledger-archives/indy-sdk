from ctypes import *
from vcx.error import VcxError, ErrorCode
from vcx.api.vcx_base import VcxBase
from vcx.common import error_message

import json


class CredentialDef(VcxBase):

    def __init__(self, source_id: str, name: str, schema_no: int):
        VcxBase.__init__(self, source_id)
        self._source_id = source_id
        self._schema_no = schema_no
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
    def schema_no(self):
        return self._schema_no

    @schema_no.setter
    def schema_no(self, x):
        self._schema_no = x

    @staticmethod
    async def create(source_id: str, name: str, schema_no: int, revocation: bool):
        constructor_params = (source_id, name, schema_no)

        c_source_id = c_char_p(source_id.encode('utf-8'))
        c_name = c_char_p(name.encode('utf-8'))
        c_schema_no = c_uint32(schema_no)
        # default institution_did in config is used as issuer_did
        c_issuer_did = None
        c_revocation = c_bool(revocation)
        c_params = (c_source_id, c_name, c_schema_no, c_issuer_did, c_revocation)

        return await CredentialDef._create("vcx_credentialdef_create",
                                           constructor_params,
                                           c_params)

    @staticmethod
    async def deserialize(data: dict):
        try:
            schema_no = data['credential_def']['ref']
            credential_def = await CredentialDef._deserialize("vcx_credentialdef_deserialize",
                                                              json.dumps(data),
                                                              data['source_id'],
                                                              data['name'],
                                                              schema_no)
            return credential_def
        except KeyError:
            raise VcxError(ErrorCode.InvalidCredentialDef, error_message(ErrorCode.InvalidCredentialDef))

    async def serialize(self) -> dict:
        return await self._serialize(CredentialDef, 'vcx_credentialdef_serialize')

    def release(self) -> None:

        self._release(CredentialDef, 'vcx_credentialdef_release')
