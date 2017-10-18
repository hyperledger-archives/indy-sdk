import asyncio
import sys

from src.cli import Cli

import logging

logging.getLogger("indy").setLevel(logging.ERROR)


def main():
    loop = asyncio.get_event_loop()
    cli = Cli(loop=loop)
    loop.run_until_complete(cli.shell(*sys.argv[1:]))


if __name__ == '__main__':
    main()
