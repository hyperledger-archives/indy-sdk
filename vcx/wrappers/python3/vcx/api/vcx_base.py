from vcx.common import do_call, create_cb, release
from ctypes import *

import logging
import json



class VcxBase:
    HANDLES = {}

    def __init__(self, source_id: str):
        self._source_id = source_id
        self._logger = logging.getLogger(__name__)
        self._handle = 0

    @property
    def handle(self):
        return self._handle

    @handle.setter
    def handle(self, handle):
        self._handle = handle

    @property
    def source_id(self):
        return self._source_id

    @source_id.setter
    def source_id(self, x):
        self._source_id = x

    @property
    def logger(self):
        return self._logger

    @classmethod
    async def _create(cls, fn: str, constructor_args, c_args):
        obj = cls(*constructor_args)

        if not hasattr(cls.create, "cb"):
            obj.logger.debug("{}: Creating callback".format(fn))
            cls.create.cb = create_cb(CFUNCTYPE(None, c_uint32, c_uint32, c_uint32))

        obj.handle = await do_call(fn,
                                   *c_args,
                                   cls.create.cb)

        VcxBase.HANDLES[obj.handle] = obj

        obj.logger.debug("created {} object".format(cls))

        return obj

    @classmethod
    async def _deserialize(cls, fn: str, data: str, *args):
        obj = cls(*args)

        if not hasattr(cls.deserialize, "cb"):
            obj.logger.debug("{}: Creating callback".format(fn))
            cls.deserialize.cb = create_cb(CFUNCTYPE(None, c_uint32, c_uint32, c_uint32))

        c_data = c_char_p(data.encode('utf-8'))

        obj.handle = await do_call(fn,
                                   c_data,
                                   cls.deserialize.cb)

        if obj.handle not in VcxBase.HANDLES:
            obj.logger.debug("deserialized {} object".format(cls))
            VcxBase.HANDLES[obj.handle] = obj
            return obj
        else:
            obj.logger.debug("duplicate {} object deserialized".format(cls))
            obj_to_return = VcxBase.HANDLES[obj.handle]
            obj.handle = 0
            return obj_to_return

    async def _serialize(self, cls, fn_str: str) -> dict:
        if not hasattr(cls.serialize, "cb"):
            self.logger.debug("{}: Creating callback".format(fn_str))
            cls.serialize.cb = create_cb(CFUNCTYPE(None, c_uint32, c_uint32, c_char_p))

        c_handle = c_uint32(self.handle)

        data = await do_call(fn_str,
                             c_handle,
                             cls.serialize.cb)

        self.logger.debug("serialized {} object".format(cls))
        j = json.loads(data.decode())
        return j

    def _release(self, cls, fn: str):
        self.logger.debug("Releasing %s handle: %s", cls, self.handle)
        if self.handle in VcxBase.HANDLES:
            c_handle = c_uint32(self.handle)
            VcxBase.HANDLES.pop(self.handle)
            release(fn, c_handle)
