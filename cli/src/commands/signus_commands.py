from src.commands.command import Command
from src.handlers.signus_commands_handler import SignusCommandHandler

handler = SignusCommandHandler()

generateKeysCmd = Command(
    id="generate_keys",
    label="generate keys",
    title="Creates keys (signing and encryption keys) for a new DID.",
    usage="generate keys [did=<did>] [seed=<seed>] [type=<crypto_type>]",
    examples=["generate keys", "generate keys did=NcYxiDXkpYi6ov5FcYDi1e",
              "generate keys seed=aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"],
    handler=handler.generate_keys_command_handler,
    pattern="(\s* (?P<generate_keys>generate\s keys) "
            "(\s+ (did\s*=\s*(?P<did>[a-zA-Z0-9]+)))? "
            "(\s+ (seed\s*=\s*(?P<seed>[a-zA-Z0-9]+)))?"
            "(\s+ (type\s*=\s*(?P<type>[a-zA-Z0-9_ ]+)))? \s*)",
    completer=["generate keys"])

replaceKeysStartCmd = Command(
    id="replace_keys_start",
    label="replace keys start",
    title=" Generated new temporary keys (signing and encryption keys) for an existing DID.",
    usage="replace keys start did=<did> [seed=<seed>] [type=<crypto_type>]",
    examples=["replace keys start did=NcYxiDXkpYi6ov5FcYDi1e",
              "replace keys start did=NcYxiDXkpYi6ov5FcYDi1e seed=aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"],
    handler=handler.replace_keys_start_command_handler,
    pattern="(\s* (?P<replace_keys_start>replace\s keys\s start)"
            "(\s+ (did\s*=\s*(?P<did>[a-zA-Z0-9]+))) "
            "(\s+ (seed\s*=\s*(?P<seed>[a-zA-Z0-9]+)))? "
            "(\s+ (type\s*=\s*(?P<type>[a-zA-Z0-9_ ]+)))? \s*) ",
    completer=["replace keys start"])

replaceKeysApplyCmd = Command(
    id="replace_keys_apply",
    label="replace keys apply",
    title="Apply temporary keys as main for an existing DID (owned by the caller of the library).",
    usage="replace keys apply did=<did>",
    examples=["replace keys apply did=NcYxiDXkpYi6ov5FcYDi1e"],
    handler=handler.replace_keys_apply_command_handler,
    pattern="(\s* (?P<replace_keys_apply>replace\s keys\s apply)"
            "(\s+ (did\s*=\s*(?P<did>[a-zA-Z0-9]+))) \s*) ",
    completer=["replace keys apply"])

storeTheirDidCmd = Command(
    id="store_their_did",
    label="store their did",
    title="Saves their DID for a pairwise connection in a secured Wallet.",
    usage="store their did <did> [verkey=<verkey>] [type=<type>]",
    examples=["store their did NcYxiDXkpYi6ov5FcYDi1e",
              "store their did NcYxiDXkpYi6ov5FcYDi1e verkey=CnEDk9HrMnmiHXEV1WFgbVCRteYnPqsJwrTdcZaNhFVW"],
    handler=handler.store_their_did_command_handler,
    pattern="(\s* (?P<store_their_did>store\s their\s did)"
            "(\s+ (?P<did>[a-zA-Z0-9]+)) "
            "(\s+ (verkey\s*=\s*(?P<verkey>[a-zA-Z0-9]+)))? "
            "(\s+ (type\s*=\s*(?P<type>[a-zA-Z0-9_ ]+)))? \s*) ",
    completer=["store their did"])

useDidCmd = Command(
    id="use_did",
    label="use DID",
    title="Set given DID as active.",
    usage="use DID <did>",
    examples=["use DID NcYxiDXkpYi6ov5FcYDi1e"],
    handler=handler.use_did_command_handler,
    pattern="(\s* (?P<use_did>use\s DID)"
            "(\s+ (?P<did>[a-zA-Z0-9]+)) \s*)",
    completer=["use DID"])

listDidCmd = Command(
    id="list_did",
    label="list DID",
    title="Lists all DIDs of active wallet.",
    usage="list DID",
    handler=handler.list_did_command_handler,
    pattern="(\s* (?P<list_did>list\s DID) \s*)",
    completer=["list DID"])

SIGNUS_COMMANDS = [generateKeysCmd, replaceKeysStartCmd, replaceKeysApplyCmd, storeTheirDidCmd, useDidCmd, listDidCmd]
