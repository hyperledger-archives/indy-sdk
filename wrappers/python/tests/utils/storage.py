from pathlib import Path
from shutil import rmtree
from tempfile import gettempdir

import logging

class StorageUtils(object):
    @staticmethod
    def cleanup():
        logger = logging.getLogger(__name__)

        tmp_path = StorageUtils.indy_temp_path()
        logger.debug("Cleaning tmp path: %s", tmp_path)

        if tmp_path.exists():
            rmtree(str(tmp_path))

        home_path = StorageUtils.indy_home_path()
        logger.debug("Cleaning home path: %s", home_path)

        if home_path.exists():
            rmtree(str(home_path))

    @staticmethod
    def indy_home_path() -> Path:
        return Path.home().joinpath(".indy")

    @staticmethod
    def indy_temp_path() -> Path:
        return Path(gettempdir()).joinpath("indy")
