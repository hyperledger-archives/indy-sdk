import sys
from prompt_toolkit.utils import is_windows, is_conemu_ansi

if is_windows():
    from prompt_toolkit.terminal.win32_output import Win32Output  # noqa
    from prompt_toolkit.terminal.conemu_output import ConEmuOutput  # noqa
else:
    from prompt_toolkit.terminal.vt100_output import Vt100_Output  # noqa


class CustomOutput(Vt100_Output):
    """
    Subclassing Vt100 just to override the `ask_for_cpr` method which prints
    an escape character on the console. Not printing the escape character
    """

    def ask_for_cpr(self):
        """
        Asks for a cursor position report (CPR).
        """
        self.flush()


def get_output_formatter():
    if is_windows():
        if is_conemu_ansi():
            return ConEmuOutput(sys.__stdout__)
        else:
            return Win32Output(sys.__stdout__)
    else:
        return CustomOutput.from_pty(sys.__stdout__, true_color=True)