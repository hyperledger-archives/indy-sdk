from src.commands.command import Command
from src.handlers.base_commands_handler import BaseCommandHandler

handler = BaseCommandHandler()

helpCmd = Command(
    id="help",
    label="help",
    title="Shows this or specific help message for given command",
    usage="help [<command name>]",
    examples=["help", "help list ids"],
    pattern="(\s* (?P<help>help) (\s+ (?P<command>[a-zA-Z0-9_ ]+))? \s*) ",
    completer=["help"])

licenseCmd = Command(
    id="license",
    label="license",
    title="Shows the license",
    usage="license",
    handler=handler.license_command_handler,
    pattern="(\s* (?P<license>license) \s*) ",
    completer=["license"])

exitCmd = Command(
    id="exit",
    label="exit",
    title="Exit the command-line interface ('quit' also works)",
    usage="exit",
    handler=handler.exit_command_handler,
    pattern="(\s* (?P<exit>exit) \s*) ",
    completer=["exit"])

BASE_COMMANDS = [helpCmd, licenseCmd, exitCmd]