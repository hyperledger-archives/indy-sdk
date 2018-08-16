from .libindy import do_call_sync

import logging
from ctypes import *
from typing import Optional
from logging import ERROR, WARNING, INFO, DEBUG

TRACE = 5


def set_logger() -> None:
    logger = logging.getLogger('Libindy')
    logging.addLevelName(TRACE, "TRACE")

    logger.debug("set_logger: >>>")

    def _log(context, level, target, args, module_path, file, line):
        libindy_logger = logger.getChild(target.decode())

        level_mapping = {1: ERROR, 2: WARNING, 3: INFO, 4: DEBUG, 5: TRACE, }

        libindy_logger.log(level_mapping[level],
                           "\t%s:%d | %s",
                           file.decode(),
                           line,
                           args.decode())

    do_call_sync('indy_set_logger',
                 None,
                 None,
                 CFUNCTYPE(None, c_void_p, c_int, c_char_p, c_char_p, c_char_p, c_char_p, c_int)(_log),
                 None)

    logger.debug("set_logger: <<<")
    return None


def set_default_logger(level: Optional[str]) -> None:
    logger = logging.getLogger("Libindy")
    logger.debug("set_default_logger: >>> level: %r",
                 level)

    c_level = c_char_p(level.encode('utf-8')) if level is not None else None

    do_call_sync('indy_set_default_logger',
                 c_level)

    logger.debug("set_default_logger: <<<")
    return None
