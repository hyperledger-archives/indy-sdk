from src.commands.command import Command
from src.handlers.wallet_commands_handler import WalletCommandHandler

handler = WalletCommandHandler()

createWalletCmd = Command(
    id="create_wallet",
    label="create wallet",
    title="Creates new wallet",
    usage="create wallet <name> [pool=<pool_name>] [type=<type>] [config={<json data>}] [credentials={<json data>}]",
    examples=["create wallet wallet1 pool=pool1", "create wallet wallet1 pool=pool1 type=default"],
    handler=handler.create_wallet_command_handler,
    pattern="(\s* (?P<create_wallet>create\s wallet) "
            "(\s+ (?P<wallet_name>[a-zA-Z0-9]+)) "
            "(\s+ (pool\s*=\s*(?P<pool_name>[a-zA-Z0-9]+)))? "
            "(\s+ (type\s*=\s*(?P<type>[a-zA-Z0-9_ ]+)))? "
            "(\s+ (config\s*=\s*(?P<config>\{{\s*.*\}})))? "
            "(\s+ (credentials\s*=\s*(?P<credentials>\{{\s*.*\}})))? \s*)".format(),
    completer={"create wallet"})

useWalletCmd = Command(
    id="use_wallet",
    label="use wallet",
    title="Use existing wallet",
    usage="use wallet <name> [config={<json data>}] [credentials={<json data>}]",
    examples=["use wallet wallet1"],
    handler=handler.use_wallet_command_handler,
    pattern="(\s* (?P<use_wallet>use\s wallet) "
            "(\s+ (?P<wallet_name>[a-zA-Z0-9]+)) "
            "(\s+ (config\s*=\s*(?P<config>\{{\s*.*\}})))? "
            "(\s+ (credentials\s*=\s*(?P<credentials>\{{\s*.*\}})))? \s*)".format(),
    completer=["use wallet"])

listWalletsCmd = Command(
    id="list_wallets",
    label="list wallets",
    title="Lists all existing wallets.",
    usage="list wallets",
    handler=handler.list_wallets_command_handler,
    pattern="(\s* (?P<list_wallets>list\s wallets) \s*)",
    completer=["list wallets"])

WALLET_COMMANDS = [createWalletCmd, useWalletCmd, listWalletsCmd]
