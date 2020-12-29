from ctypes import *

import logging
from logging import ERROR, WARNING, INFO, DEBUG

TRACE = 5


def set_logger(cdll):
    logger = logging.getLogger(__name__)
    logging.addLevelName(TRACE, "TRACE")

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

    level_mapping = {ERROR: 1, WARNING: 2, INFO: 3, DEBUG: 4, TRACE: 5}
    c_level = c_uint32(level_mapping.get(logging.root.level) or 0)

    getattr(cdll, 'vcx_set_logger_with_max_lvl')(None,
                                                 set_logger.callbacks['enabled_cb'],
                                                 set_logger.callbacks['log_cb'],
                                                 set_logger.callbacks['flush_cb'],
                                                 c_level)

    logger.debug("set_logger: <<<")
