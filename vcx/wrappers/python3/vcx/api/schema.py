from ctypes import *
from vcx.common import do_call, create_cb
from vcx.error import VcxError, ErrorCode, error_message
from vcx.api.vcx_base import VcxBase

import json


class Schema(VcxBase):
    """
    Object that represents a schema written on the ledger.

    Attributes:
        source_id: user generated unique identifier
        schema_id: the ledger ID of the schema
        attrs: attribute/value pairs (the number of attributes should be less or equal than 125)
        version: version of the schema
    """

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
        """
        Creates a new schema object that is written to the ledger

        :param source_id: Institution's unique ID for the schema
        :param name: Name of schema
        :param version: Version of the schema
        :param attrs: Atttributes of the schema
        :param payment_handle: NYI - payment of ledger fee is taken from wallet automatically
        Example:
        source_id = 'foobar123'
        name = 'Address Schema'
        version = '1.0'
        attrs = ['address', 'city', 'state']
        payment_handle = 0
        schema = await Schema.create(source_id, name, version, attrs, payment_handle)
        :return: schema object, written to ledger
        """
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
        """
        Create the object from a previously serialized object.

        :param data: The output of the "serialize" call
        Example:
        source_id = 'foobar123'
        name = 'Address Schema'
        version = '1.0'
        attrs = ['address', 'city', 'state']
        payment_handle = 0
        schema1 = await Schema.create(source_id, name, version, attrs, payment_handle)
        data1 = await schema1.serialize()
        :return: A re-instantiated object
        """
        try:
            # Todo: Find better way to access attr_names. Potential for issues.
            schema = await Schema._deserialize("vcx_schema_deserialize",
                                               json.dumps(data),
                                               data['data']['source_id'],
                                               data['data']['name'],
                                               data['data']['version'],
                                               data['data']['data'])

            schema.schema_id = await schema.get_schema_id()
            return schema
        except KeyError:
            raise VcxError(ErrorCode.InvalidSchema)

    @staticmethod
    async def lookup(source_id: str, schema_id: str):
        """
        Create a new schema object from an existing ledger schema

        :param source_id: Institution's personal identification for the schema
        :param schema_id: Ledger schema ID for lookup
        Example:
        source_id = 'foobar123'
        name = 'Address Schema'
        version = '1.0'
        attrs = ['address', 'city', 'state']
        payment_handle = 0
        schema1 = await Schema.create(source_id, name, version, attrs, payment_handle)
        id1 = await schema.get_schema_id()
        data = await Schema.lookup(source_id, schema_id)
        assert data.attrs.sort() == ['sex', 'age', 'name', 'height'].sort()
        assert data.name == 'test-licence'
        assert data.handle > 0
        :return: schema object
        """
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
        """
        Serialize the object for storage

        Example:
        source_id = 'foobar123'
        name = 'Address Schema'
        version = '1.0'
        attrs = ['address', 'city', 'state']
        payment_handle = 0
        schema1 = await Schema.create(source_id, name, version, attrs, payment_handle)
        data1 = await schema1.serialize()
        :return: serialized object
        """
        return await self._serialize(Schema, 'vcx_schema_serialize')

    def release(self) -> None:
        """
        destroy the object and release any memory associated with it

        :return: None
        """
        self._release(Schema, 'vcx_schema_release')

    async def get_schema_id(self):
        """
        Get the ledger ID of the object

        Example:
        source_id = 'foobar123'
        name = 'Address Schema'
        version = '1.0'
        attrs = ['address', 'city', 'state']
        payment_handle = 0
        schema1 = await Schema.create(source_id, name, version, attrs, payment_handle)
        id1 = await schema.get_schema_id()
        :return: ID string
        """
        cb = create_cb(CFUNCTYPE(None, c_uint32, c_uint32, c_char_p))
        c_handle = c_uint32(self.handle)
        id = await do_call('vcx_schema_get_schema_id', c_handle, cb)
        return id.decode()

    async def get_payment_txn(self):
        """
        Get the payment transaction information generated when paying the ledger fee

        Example:
        source_id = 'foobar123'
        name = 'Address Schema'
        version = '1.0'
        attrs = ['address', 'city', 'state']
        payment_handle = 0
        schema1 = await Schema.create(source_id, name, version, attrs, payment_handle)
        txn = await schema1.get_payment_txn()
        :return: JSON object with input address and output UTXOs
        """
        if not hasattr(Schema.get_payment_txn, "cb"):
            self.logger.debug("vcx_schema_get_payment_txn: Creating callback")
            Schema.get_payment_txn.cb = create_cb(CFUNCTYPE(None, c_uint32, c_uint32, c_char_p))

        c_credential_handle = c_uint32(self.handle)

        payment_txn = await do_call('vcx_schema_get_payment_txn',
                      c_credential_handle,
                      Schema.get_payment_txn.cb)

        return json.loads(payment_txn.decode())
