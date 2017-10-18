from src.commands.command import Command
from src.handlers.ledger_commands_handler import LedgerCommandHandler

handler = LedgerCommandHandler()

sendMessageCmd = Command(
    id="send",
    label="send",
    title="Sends a message to pool.",
    usage="send {<json request data>}",
    examples=["send {'type':105, 'dest': 'Th7MpTaRZVRYnPiabds81Y',}"],
    handler=handler.send_message_command_handler,
    pattern="(\s* (?P<send>send) "
            "(\s+ (?P<request>\{{\s*.*\}})) \s*)".format(),
    completer=["send"])

sendNymCmd = Command(
    id="send_nym",
    label="send NYM",
    title="Builds ans sends a NYM request to Ledger.",
    usage="send NYM <did> [verkey=<verkey>] [alias=<alias>] [role=<role>]",
    examples=["send NYM NcYxiDXkpYi6ov5FcYDi1e",
              "send NYM NcYxiDXkpYi6ov5FcYDi1e verkey=CnEDk9HrMnmiHXEV1WFgbVCRteYnPqsJwrTdcZaNhFVW",
              "send NYM NcYxiDXkpYi6ov5FcYDi1e verkey=CnEDk9HrMnmiHXEV1WFgbVCRteYnPqsJwrTdcZaNhFVW role=TRUSTEE"],
    handler=handler.send_nym_command_handler,
    pattern="(\s* (?P<send_nym>send\s NYM) "
            "(\s+ (?P<did>[a-zA-Z0-9]+)) "
            "(\s+ (verkey\s*=\s*(?P<verkey>[a-zA-Z0-9]+)))?"
            "(\s+ (alias\s*=\s*(?P<alias>[a-zA-Z0-9]+)))?"
            "(\s+ (role\s*=\s*(?P<role>[a-zA-Z0-9_ ]+)))? \s*)",
    completer=["send NYM"])

LEDGER_COMMANDS = [sendMessageCmd, sendNymCmd]