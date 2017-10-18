from src.commands.command import Command
from src.handlers.pool_commands_handler import PoolCommandHandler

handler = PoolCommandHandler()

refreshPoolCmd = Command(
    id="refresh_pool",
    label="refresh pool",
    title="Refreshes a local copy of a pool ledger and updates pool nodes connections.",
    usage="refresh pool",
    examples=["refresh pool"],
    handler=handler.refresh_pool_command_handler,
    pattern="(\s* (?P<refresh_pool>refresh\s pool) \s*)",
    completer={"refresh pool"})

poolWalletsCmd = Command(
    id="list_pools",
    label="list pools",
    title="Lists all existing pools.",
    usage="list pools",
    handler=handler.list_pools_command_handler,
    pattern="(\s* (?P<list_pools>list\s pools) \s*)",
    completer=["list pools"])

useWalletCmd = Command(
    id="connect_pool",
    label="connect pool",
    title="Connect to existing pool",
    usage="connect pool <pool_name> [config={<json data>}]",
    examples=["connect pool pool1"],
    handler=handler.connect_pool_command_handler,
    pattern="(\s* (?P<connect_pool>connect\s pool) "
            "(\s+ (?P<pool_name>[a-zA-Z0-9]+)) "
            "(\s+ (config\s*=\s*(?P<config>\{{\s*.*\}})))? \s*)".format(),
    completer=["connect pool"])

POOL_COMMANDS = [refreshPoolCmd, poolWalletsCmd]
