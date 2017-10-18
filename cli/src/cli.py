from __future__ import unicode_literals

import json

from indy import error
from prompt_toolkit.contrib.completers import WordCompleter
from prompt_toolkit.contrib.regular_languages.compiler import compile
from prompt_toolkit.contrib.regular_languages.completion import GrammarCompleter
from prompt_toolkit.contrib.regular_languages.lexer import GrammarLexer
from prompt_toolkit.interface import CommandLineInterface
from prompt_toolkit.layout.lexers import SimpleLexer
from prompt_toolkit.shortcuts import create_prompt_application, \
    create_asyncio_eventloop
from prompt_toolkit.styles import PygmentsStyle
from pygments.token import Token

from src.commands.base_commands import BASE_COMMANDS
from src.commands.ledger_commands import LEDGER_COMMANDS
from src.commands.pool_commands import POOL_COMMANDS
from src.commands.signus_commands import SIGNUS_COMMANDS
from src.commands.wallet_commands import WALLET_COMMANDS
from src.constants import CLI_STYLE, CLI_NAME, VERSION
from src.handlers.base_commands_handler import BaseCommandHandler
from src.handlers.pool_commands_handler import PoolCommandHandler
from src.handlers.wallet_commands_handler import WalletCommandHandler
from src.utils.environment import state_file
from src.utils.errors import Exit, indy_error_to_message
from src.utils.output import get_output_formatter
from src.utils.state import State

COMMANDS = BASE_COMMANDS + WALLET_COMMANDS + SIGNUS_COMMANDS + POOL_COMMANDS + LEDGER_COMMANDS


class Cli:
    # noinspection PyPep8
    def __init__(self, loop):
        self.state = State()

        self._grammar = compile(" |".join(self.grammars))
        self._lexer = GrammarLexer(self._grammar, lexers=self.lexers)
        self._completer = GrammarCompleter(self._grammar, self.completers)
        self._style = PygmentsStyle.from_defaults(CLI_STYLE)
        self._action_handlers = {cmd.id: cmd.handler for cmd in COMMANDS}

        self._base_handler = BaseCommandHandler(COMMANDS)
        self._pool_handler = PoolCommandHandler()
        self._wallet_handler = WalletCommandHandler()

        event_loop = create_asyncio_eventloop(loop)

        app = create_prompt_application('{}> '.format(CLI_NAME.lower()),
                                        lexer=self._lexer,
                                        completer=self._completer,
                                        style=self._style)

        out = get_output_formatter()

        self.cli = CommandLineInterface(application=app,
                                        eventloop=event_loop,
                                        output=out)

        print("\n{}-CLI (c) 2017 Hyperladger.\n"
              "Type 'help' for more information."
              "Running {} {}\n".format(CLI_NAME, CLI_NAME, VERSION))

        loop.run_until_complete(self.init())

    @property
    def grammars(self):
        return [cmd.pattern for cmd in COMMANDS]

    @property
    def completers(self):
        return {cmd.id: WordCompleter(cmd.completer) for cmd in COMMANDS}

    @property
    def lexers(self):
        return {cmd.id: SimpleLexer(Token.Keyword) for cmd in COMMANDS}

    async def shell(self, interactive=True):
        """
        Coroutine that runs command, including those from an interactive
        command line.

        :param interactive: when True, this coroutine will process commands
            entered on the command line.
        :return:
        """
        while interactive:
            try:
                result = await self.cli.run_async()
                cmd = result.text.strip() if result else ""
                await self.parse(cmd)
            except (EOFError, KeyboardInterrupt, Exit):
                await self.close()
                break

        print('Goodbye.')

    async def init(self):
        try:
            if state_file().exists():
                with open(str(state_file())) as data_file:
                    self.state = State.from_json(json.load(data_file))

            await self._pool_handler.init_pool(self.state)
            await self._wallet_handler.init_wallet(self.state)
        except error.IndyError as err:
            print(indy_error_to_message(err.error_code))
        except IOError as err:
            print(str(err))

    async def close(self):
        try:
            await self._wallet_handler.close_wallet(self.state, None)
            await self._pool_handler.close_pool(self.state, None)

            with open(str(state_file()), 'w') as outfile:
                json.dump(self.state.to_json(), outfile)
        except error.IndyError as err:
            print(indy_error_to_message(err.error_code))
        except IOError as err:
            print(str(err))

    async def parse(self, cmd_text):
        cmd = self._grammar.match(cmd_text.strip())
        if not cmd:
            return self._base_handler.handle_invalid_command(cmd_text)

        # TODO: problem with cross relationship
        if cmd.variables().get('help'):
            return await self._base_handler.help_command_handler(cmd.variables())

        handler = next(
            (self._action_handlers[action] for action in self._action_handlers.keys() if cmd.variables().get(action)),
            None)

        if not handler:
            return self._base_handler.handle_invalid_command(cmd_text)

        try:
            await handler(self.state, cmd.variables())
        except error.IndyError as err:
            print(indy_error_to_message(err.error_code))
        except (EOFError, KeyboardInterrupt, Exit) as err:
            raise err
        except Exception as err:
            print(str(err))
