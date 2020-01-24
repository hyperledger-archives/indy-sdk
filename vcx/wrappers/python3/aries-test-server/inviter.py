import json

from vcx.api.connection import Connection
from utils import init_vcx, run_coroutine_in_new_loop
from connection import BaseConnection


class Inviter(BaseConnection):
    async def start(self):
        await init_vcx()
        print("Create a connection to alice and print out the invite details")
        connection_ = await Connection.create('alice')
        await connection_.connect('{"use_public_did": true}')
        await connection_.update_state()
        details = await connection_.invite_details(False)
        print("**invite details**")
        print(json.dumps(details))
        print("******************")

        self.connection_data = await connection_.serialize()
        connection_.release()

        return json.dumps(details)


    def connect(self):
        run_coroutine_in_new_loop(self.update_state)
