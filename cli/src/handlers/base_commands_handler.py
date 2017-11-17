from src.constants import CLI_NAME
from src.utils.errors import Exit


class BaseCommandHandler:
    def __init__(self, commands=None):
        self.commands = commands or []

    async def help_command_handler(self, matched_vars):
        requested_command = matched_vars.get('command')
        if requested_command:
            help_message = self.get_help_message(requested_command)
            if help_message:
                print(str(help_message))
            else:
                print("No such command found: {}\n".format(requested_command))
                self.print_help()
        else:
            self.print_help()

    def get_help_message(self, command):
        return next((hm for hm in self.commands if hm.label == command), None)

    def print_help(self):
        message = "{}-CLI, a simple command-line interface for a {}.".format(CLI_NAME, CLI_NAME)
        message += "\nUsage:  help [<command name>] \n"

        for cmd in self.commands:
            message += "\n     {} - {}".format(cmd.label, cmd.title)

        print("\n{}\n".format(message))

    @staticmethod
    async def license_command_handler(info, matched_vars):
        print("""
                            Copyright 2016 Evernym, Inc.
            Licensed under the Apache License, Version 2.0 (the "License");
            you may not use this file except in compliance with the License.
            You may obtain a copy of the License at

                http://www.apache.org/licenses/LICENSE-2.0

            Unless required by applicable law or agreed to in writing, software
            distributed under the License is distributed on an "AS IS" BASIS,
            WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
            See the License for the specific language governing permissions and
            limitations under the License.
            """)

    @staticmethod
    async def exit_command_handler(info, matched_vars):
        raise Exit

    def handle_invalid_command(self, cmd_text):
        help_message = self.get_help_message(cmd_text)
        if help_message:
            print("Invalid syntax: '{}'".format(cmd_text))
            print(str(help_message))
        else:
            print("Invalid command: '{}'".format(cmd_text))
            self.print_help()
