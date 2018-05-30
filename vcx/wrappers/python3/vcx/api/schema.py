from ctypes import *
from vcx.common import do_call, create_cb, error_message
from vcx.error import VcxError, ErrorCode
from vcx.api.vcx_base import VcxBase

import json


class Schema(VcxBase):

    def __init__(self, source_id: str, name: str, version: str, attrs: list):
        VcxBase.__init__(self, source_id)
        self._source_id = source_id
        self._schema_id = None
        self._attrs = attrs
        self._name = name
        self._version = version

    def __del__(self):
        self.release()
        self.logger.debug("Deleted {} obj: {}".format(Schema, self.handle))

    @property
    def schema_id(self):
        return self._schema_id

    @schema_id.setter
    def schema_id(self, x):
        self._schema_id = x

    @property
    def name(self):
        return self._name

    @name.setter
    def name(self, x):
        self._name = x

    @property
    def attrs(self):
        return self._attrs

    @attrs.setter
    def attrs(self, x):
        self._attrs = x

    @property
    def version(self):
        return self._version

    @version.setter
    def version(self, x):
        self._version = x

    @staticmethod
    async def create(source_id: str, name: str, version: str, attrs: list, payment_handle: int):
        constructor_params = (source_id, name, version, attrs)

        c_source_id = c_char_p(source_id.encode('utf-8'))
        c_name = c_char_p(name.encode('utf-8'))
        c_version = c_char_p(version.encode('utf-8'))
        c_schema_data = c_char_p(json.dumps(attrs).encode('utf-8'))
        c_payment = c_uint32(payment_handle)
        c_params = (c_source_id, c_name, c_version, c_schema_data, c_payment)

        schema = await Schema._create("vcx_schema_create", constructor_params, c_params)
        schema.schema_id = await schema.get_schema_id()
        return schema

    @staticmethod
    async def deserialize(data: dict):
        try:
            # Todo: Find better way to access attr_names. Potential for issues.
            schema = await Schema._deserialize("vcx_schema_deserialize",
                                               json.dumps(data),
                                               data['source_id'],
                                               data['name'],
                                               data['version'],
                                               data['data'])

            schema.schema_id = await schema.get_schema_id()
            return schema
        except KeyError:
            raise VcxError(ErrorCode.InvalidSchema, error_message(ErrorCode.InvalidSchema))

    @staticmethod
    async def lookup(source_id: str, schema_id: str):
        try:
            schema = Schema(source_id, '', '', [])

            if not hasattr(Schema.lookup, "cb"):
                schema.logger.debug("vcx_schema_get_attributes: Creating callback")
                Schema.lookup.cb = create_cb(CFUNCTYPE(None, c_uint32, c_uint32, c_uint32, c_char_p))

            c_source_id = c_char_p(source_id.encode('utf-8'))
            c_schema_id = c_char_p(schema_id.encode('utf-8'))

            handle, data = await do_call('vcx_schema_get_attributes',
                                         c_source_id,
                                         c_schema_id,
                                         Schema.lookup.cb)
            schema.logger.debug("created schema object")

            schema_result = json.loads(data.decode())
            schema.attrs = schema_result['data']
            schema.name = schema_result['name']
            schema.version = schema_result['version']
            schema.handle = handle
            return schema
        except KeyError:
            raise VcxError(ErrorCode.InvalidSchema)

    async def serialize(self) -> dict:
        return await self._serialize(Schema, 'vcx_schema_serialize')

    def release(self) -> None:
        self._release(Schema, 'vcx_schema_release')

    async def get_schema_id(self):
        cb = create_cb(CFUNCTYPE(None, c_uint32, c_uint32, c_char_p))
        c_handle = c_uint32(self.handle)
        id = await do_call('vcx_schema_get_schema_id', c_handle, cb)
        return id.decode()
