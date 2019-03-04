from ctypes import *

import logging
from logging import ERROR, WARNING, INFO, DEBUG, CRITICAL

TRACE = 5


def set_logger(cdll):
    logger = logging.getLogger(__name__)
    logging.addLevelName(TRACE, "TRACE")
    logging.basicConfig(level=CRITICAL)

    logger.debug("set_logger: >>>")

    def _log(context, level, target, message, module_path, file, line):
        libvcx_logger = logger.getChild('native.' + target.decode().replace('::', '.'))

        level_mapping = {1: ERROR, 2: WARNING, 3: INFO, 4: DEBUG, 5: TRACE, }

        libvcx_logger.log(level_mapping[level],
                          "\t%s:%d | %s",
                          file.decode(),
                          line,
                          message.decode())

    set_logger.callbacks = {
        'enabled_cb': None,
        'log_cb': CFUNCTYPE(None, c_void_p, c_int, c_char_p, c_char_p, c_char_p, c_char_p, c_int)(_log),
        'flush_cb': None
    }

    getattr(cdll, 'vcx_set_logger')(None,
                                    set_logger.callbacks['enabled_cb'],
                                    set_logger.callbacks['log_cb'],
                                    set_logger.callbacks['flush_cb'])

    logger.debug("set_logger: <<<")
