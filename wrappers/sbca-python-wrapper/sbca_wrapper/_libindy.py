import itertools
import json
import logging
import sys
from asyncio import AbstractEventLoop, Future, get_event_loop
from ctypes import CDLL, CFUNCTYPE, byref, c_char_p, c_int, c_void_p
from typing import Any, Callable, Dict, Tuple

from .error import LibindyError, CommonInvalidParamError, error_code_map

# Setup Logger
_handler = logging.StreamHandler()
_formatter = logging.Formatter('[%(levelname)s] %(name)s | %(message)s')
_handler.setFormatter(_formatter)

LOGGER: logging.Logger = logging.getLogger('libindy')
NATIVE_LOGGER: logging.Logger = LOGGER.getChild(suffix='native')
LOGGER.setLevel(logging.DEBUG)
LOGGER.addHandler(_handler)
LOGGER.propagate = False


class Libindy:
    """Holds the functions for Libindy interactions."""

    _INSTANCE: 'Libindy' = None
    _LIBRARY: CDLL = None

    _FUTURES: Dict[int, Tuple[AbstractEventLoop, Future]] = {}
    _COMMAND_HANDLE_GENERATOR: itertools.count = itertools.count()
    _FUTURES_COUNTER: itertools.count = itertools.count()

    _CAN_SET_RUNTIME_CONFIG = True

    # -------------------------------------------------------------------------
    #  Constructor
    # -------------------------------------------------------------------------
    def __new__(cls) -> 'Libindy':
        """Creates or returns a Singleton instance of Libindy."""

        if not Libindy._INSTANCE:
            LOGGER.info('Building Libindy instance...')
            cls._INSTANCE = object.__new__(cls)
            cls._LIBRARY = cls._load_library()
            cls._set_native_logger()
            LOGGER.info('Libindy setup complete.')

        return cls._INSTANCE

    # -------------------------------------------------------------------------
    #  Properties
    # -------------------------------------------------------------------------
    @property
    def logger(self) -> logging.Logger:
        return LOGGER

    @property
    def native_logger(self) -> logging.Logger:
        return NATIVE_LOGGER

    # -------------------------------------------------------------------------
    #  Methods
    # -------------------------------------------------------------------------

    # Libindy Setup -----------------------------------------------------------
    @staticmethod
    def _load_library() -> CDLL:

        LOGGER.info('   Loading C-library...')
        library_name = {
            'darwin': 'libindy.dylib',
            'linux': 'libindy.so',
            'linux2': 'libindy.so',
            'win32': 'indy.dll'
        }.get(sys.platform)

        if not library_name:
            raise OSError(
                f'This OS is not supported by Libindy: {sys.platform}!'
            )

        try:
            return CDLL(name=library_name)
        except OSError:
            LOGGER.error('Could not load Libindy from your system!')
            raise

    @classmethod
    def _set_native_logger(cls):

        LOGGER.info('   Setting native logger...')
        logging.addLevelName(level=5, levelName='TRACE')
        log_level_map = {
            1: logging.ERROR,
            2: logging.WARNING,
            3: logging.INFO,
            4: logging.DEBUG,
            5: 5
        }

        def _log(context, level, target, message, module_path, file, line):
            _logger = NATIVE_LOGGER.getChild(
                suffix=target.decode().replace("::", ".")
            )

            _logger.log(
                level=log_level_map[level],
                msg=f'{file.decode()}:{line} | {message.decode()}'
            )

        cls._native_logger_callback = CFUNCTYPE(None, c_void_p, c_int,
                                                c_char_p, c_char_p, c_char_p,
                                                c_char_p, c_int)(_log)
        getattr(cls._LIBRARY, 'indy_set_logger')(None, None,
                                                 cls._native_logger_callback,
                                                 None)

    def set_runtime_config(self, thread_pool_size: int = 4,
                           collect_backtrace: bool = True):
        """Sets the runtime config for the C-library.

        NOTE: This function has to be run before running any other Libindy
            functions!

        TODO: Set with environment variables?

        :param thread_pool_size  : The maximal amount of threads Libindy
            creates for crypto operations.
            Optional; Defaults to: `4`
        :param collect_backtrace : Whether to collect the backtrace if an error
            occurs.
            Optional; Defaults to: `True`
        """

        if not self._CAN_SET_RUNTIME_CONFIG:
            raise RuntimeError('The runtime configurations have to be set '
                               'before calling the library!')

        LOGGER.info(f'Setting runtime config >>> '
                    f'thread_pool_size={thread_pool_size}, '
                    f'collect_backtrace={collect_backtrace}')

        config = {
            'crypto_thread_pool_size': thread_pool_size,
            'collect_backtrace': collect_backtrace
        }

        getattr(self._LIBRARY, 'indy_set_runtime_config')(
            json.dumps(config).encode(encoding='utf-8')
        )

    # Libindy Command Running -------------------------------------------------
    def __call__(self, command_name: str, *command_args) -> Future:
        """Calls a function in the C-library.

        :param command_name : The name of the command to call.
        :param command_args : The C-type encoded arguments of the command.

        :returns: The command response wrapped as an asyncio.Future object.

        :raises NotImplementedError: Raised if the C-Library does not implement
            the command with the name `command_name`.
        """

        if not hasattr(self._LIBRARY, command_name):
            raise NotImplementedError(f'Libindy does not implement this '
                                      f'command: {command_name}!')

        self._CAN_SET_RUNTIME_CONFIG = False

        loop = get_event_loop()
        command_future = loop.create_future()
        command_handle = next(self._FUTURES_COUNTER)
        self._FUTURES[command_handle] = (loop, command_future)

        response_code: int = getattr(self._LIBRARY, command_name)(
            command_handle, *command_args
        )

        if response_code != 0:
            LOGGER.error(f'Libindy responded with code {response_code}!')
            command_future.set_exception(self._get_indy_error(response_code))

        return command_future

    def _run_callback(self, command_handle: int, response: LibindyError,
                      *response_values):
        loop, _ = self._FUTURES[command_handle]
        loop.call_soon_threadsafe(self._loop_callback, command_handle,
                                  response, *response_values)

    def _loop_callback(self, command_handle: int, response: LibindyError,
                       *response_values):

        _, future = self._FUTURES.pop(command_handle)
        response_values = None if not response_values else response_values

        if not future.cancelled():
            if response.indy_code == 0:
                future.set_result(response_values)
            else:
                future.set_exception(response)
        else:
            LOGGER.warning('Future was cancelled before callback execution!\n')

    def _get_indy_error(self, response_code: int) -> LibindyError:

        # Return success
        if response_code == 0:
            return LibindyError(0, 'Success')

        # Get and return error data
        c_error = c_char_p()
        getattr(self._LIBRARY, 'indy_get_current_error')(byref(c_error))
        error_details: dict = json.loads(c_error.value.decode())

        # Get error type
        if response_code not in error_code_map.keys():
            raise KeyError(f'Libindy responded with unknown response code: '
                           f'{response_code}!')

        error_type: type = error_code_map.get(response_code)

        # Create and return error
        if error_type is CommonInvalidParamError:
            return error_type(response_code, error_details.get('message'),
                              error_details.get('backtrace'))
        return error_type(error_details.get('message'),
                          error_details.get('backtrace'))

    def create_callback(self, cb_signature: CFUNCTYPE,
                        cb_transform_fn: Callable = None) -> Any:
        """Creates the callback function of a Libindy command.

        :param cb_signature    : The C-signature of the callback function.
        :param cb_transform_fn : The callback transform function.

        :returns: The callback function.
        """

        def callback(handle: int, code: int, *values) -> Any:
            if cb_transform_fn:
                values = cb_transform_fn(*values)
            response = self._get_indy_error(code)
            self._run_callback(handle, response, *values)

        return cb_signature(callback)

    def implements_command(self, command_name: str) -> bool:
        """Checks if Libindy implements a specific command.

        :param command_name: The name of the command.

        :returns: Whether the command is implemented in Libindy.
        """
        return hasattr(self._LIBRARY, command_name)

    # Various -----------------------------------------------------------------


LIBINDY: Libindy = Libindy()
