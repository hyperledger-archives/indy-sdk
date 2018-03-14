import inspect
from ctypes import CFUNCTYPE, c_int32, c_char_p

from indy.libindy import create_cb, do_call


def log_args(logger):
    frame = inspect.currentframe()
    caller = inspect.getouterframes(frame, 2)[1]
    f_name = caller.function
    res = inspect.getargvalues(caller.frame)
    locals = res.locals
    passed_args = {
        **{nm: locals[nm] for nm in res.args or []},
        **{nm: locals[nm] for nm in res.varargs or []},
        **{nm: locals[nm] for nm in res.keywords or []},
    }
    log_string = '{}: >>> '.format(f_name) + ', '.join(['{}: {}, '.format(k, v) for k, v in passed_args.items()])
    logger.debug(log_string)

