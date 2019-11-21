import json

from vcx.api.connection import Connection
from utils import init_vcx, run_coroutine_in_new_loop
from connection import BaseConnection


class Invitee(BaseConnection):
    def __init__(self, invite):
        BaseConnection.__init__(self)
        invite = json.loads(invite)
        self.invite = json.dumps(invite)


    async def start(self):
        await init_vcx()
        connection_ = await Connection.create_with_details('faber', self.invite)

        self.connection_data = await connection_.serialize()
        connection_.release()
        return ""


    async def _connect(self):
        connection_ = await Connection.deserialize(self.connection_data)
        await connection_.connect('{"use_public_did": true}')
        await connection_.update_state()
        self.connection_data = await connection_.serialize()
        connection_.release()


    def connect(self):
        run_coroutine_in_new_loop(self._connect)
        run_coroutine_in_new_loop(self.update_state)
