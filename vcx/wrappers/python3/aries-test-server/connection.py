from enum import Enum
from time import sleep

from vcx.api.connection import Connection
from vcx.state import State


class BaseConnection:
    def __init__(self):
        self.connection_data = {}
        self.active = True
        self.thread = None


    async def update_state(self):
        connection_ = await Connection.deserialize(self.connection_data)

        print("Poll agency and wait for alice to accept the invitation (start alice.py now)")
        connection_state = await connection_.get_state()
        while connection_state != State.Accepted and self.active:
            sleep(2)
            await connection_.update_state()
            self.connection_data = await connection_.serialize()
            connection_state = await connection_.get_state()
        connection_.release()


    async def get_state(self):
        connection_ = await Connection.deserialize(self.connection_data)
        connection_state = await connection_.get_state()
        connection_.release()
        return ConnectionStatus.from_vcx_state(connection_state).value


    async def stop(self):
        self.active = False
        self.thread.join()


class ConnectionStatus(Enum):
    NULL = "null"
    INVITED = "invited"
    REQUESTED = "requested"
    RESPONDED = "responded"
    COMPLETE = "complete"


    @staticmethod
    def from_vcx_state(state):
        if state == State.RequestReceived:
            return ConnectionStatus.REQUESTED
        elif state == State.OfferSent:
            return ConnectionStatus.RESPONDED
        elif state == State.Accepted:
            return ConnectionStatus.COMPLETE
        else:
            return ConnectionStatus.NULL
