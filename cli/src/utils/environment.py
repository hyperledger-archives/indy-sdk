import os
from pathlib import Path
from tempfile import gettempdir

from src.constants import WORK_DIR, STATE_FILE


def home_dir():
    return Path.home().joinpath(WORK_DIR)


def temp_dir():
    return Path(gettempdir()).joinpath(WORK_DIR)


def wallet_dir():
    return home_dir().joinpath("wallet/")


def pool_dir():
    return home_dir().joinpath("pool/")


def get_files(path):
    return [name for name in os.listdir(str(path))]


def state_file():
    return home_dir().joinpath(STATE_FILE)
