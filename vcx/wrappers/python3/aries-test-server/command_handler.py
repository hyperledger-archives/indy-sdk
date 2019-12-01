import threading
from enum import Enum

from inviter import Inviter
from invitee import Invitee
from utils import run_coroutine


class Action(Enum):
    START_CONNECTION_AS_INVITER = "start_connection_as_inviter"
    START_CONNECTION_AS_INVITEE = "start_connection_as_invitee"
    GET_CONNECTION_STATE = "get_connection_state"
    RESET_CONNECTION = "reset_connection"


class CommandHandler:
    def __init__(self):
        self.state = {}


    def handle_command(self, command: dict) -> str:
        if command['action'] == Action.START_CONNECTION_AS_INVITER.value:
            return self.handle_start_connection_as_inviter()
        elif command['action'] == Action.START_CONNECTION_AS_INVITEE.value:
            return self.handle_start_connection_as_invitee(command['payload'])
        elif command['action'] == Action.GET_CONNECTION_STATE.value:
            return self.handle_get_connection_state()
        elif command['action'] == Action.RESET_CONNECTION.value:
            return self.handle_reset_connection()
        else:
            return "Unexpected action"


    def handle_start_connection_as_inviter(self) -> str:
        invite = self.start_connection(Inviter())
        return invite


    def handle_start_connection_as_invitee(self, invite) -> str:
        self.start_connection(Invitee(invite))
        return "OK"


    def start_connection(self, actor):
        result = run_coroutine(actor.start)
        actor.thread = threading.Thread(target=actor.connect, args=())
        actor.thread.start()
        self.state['connection'] = actor
        return result


    def handle_get_connection_state(self) -> str:
        return run_coroutine(self.state['connection'].get_state)


    def handle_reset_connection(self) -> str:
        run_coroutine(self.state['connection'].stop)
        return "OK"
