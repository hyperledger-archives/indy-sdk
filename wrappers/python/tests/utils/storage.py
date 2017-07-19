from pathlib import Path
from shutil import rmtree
from tempfile import gettempdir

import os
import logging


def indy_temp_path() -> Path:
    logger = logging.getLogger(__name__)
    logger.debug("indy_temp_path: >>>")

    res = Path(gettempdir()).joinpath("indy")

    logger.debug("indy_temp_path: <<< res: %s", res)
    return res


def indy_home_path() -> Path:
    logger = logging.getLogger(__name__)
    logger.debug("indy_home_path: >>>")

    res = Path.home().joinpath(".indy")

    logger.debug("indy_home_path: <<< res: %s", res)
    return res


def create_temp_dir():
    os.makedirs(str(indy_temp_path()))


def cleanup():
    logger = logging.getLogger(__name__)
    logger.debug("cleanup: >>>")

    tmp_path = indy_temp_path()

    if tmp_path.exists():
        logger.debug("cleanup: Cleaning tmp path: %s", tmp_path)
        rmtree(str(tmp_path))

    home_path = indy_home_path()

    if home_path.exists():
        logger.debug("cleanup: Cleaning home path: %s", home_path)
        rmtree(str(home_path))

    logger.debug("cleanup: <<<")
