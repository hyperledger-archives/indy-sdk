from vcx.common import do_call, create_cb
from ctypes import *
from vcx.api.vcx_base import VcxBase


class VcxStateful(VcxBase):

    def __init__(self, source_id: str):
        VcxBase.__init__(self, source_id)

    async def _update_state(self, cls, fn: str) -> int:
        if not hasattr(cls.update_state, "cb"):
            self.logger.debug("{}: Creating callback".format(fn))
            cls.update_state.cb = create_cb(CFUNCTYPE(None, c_uint32, c_uint32, c_uint32))

        c_handle = c_uint32(self.handle)

        state = await do_call(fn,
                              c_handle,
                              cls.update_state.cb)

        self.logger.debug("{} object has state of: {}".format(cls, state))
        return state

    async def _get_state(self, cls, fn: str) -> int:
        if not hasattr(cls.get_state, "cb"):
            self.logger.debug("{}: Creating callback".format(fn))
            cls.get_state.cb = create_cb(CFUNCTYPE(None, c_uint32, c_uint32, c_uint32))


        c_handle = c_uint32(self.handle)

        return await do_call(fn,
                             c_handle,
                             cls.get_state.cb)
