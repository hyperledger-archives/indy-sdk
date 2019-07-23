import inspect
import json
import time
from ctypes import (CFUNCTYPE, POINTER, c_bool, c_char_p, c_int32, c_uint8,
                    c_uint32)
from functools import wraps
from typing import Any, Callable, Dict, Optional, Tuple, Union

from ._libindy import LIBINDY

_LIBINDY_LOGGER = LIBINDY.logger
_LOGGER = _LIBINDY_LOGGER.getChild('command')


class LibindyCommand:
    """Designates a function as a Libindy command.
    ---------------------------------------------------------------------------
    This class is meant to be used as a decorator on a function. It will
    generate a function body that is capable of calling the Libindy C-library.
    Any existing function body will be overwritten.
    """

    def __init__(
            self,
            command_name: str,
            return_type: Optional[Tuple] = None,
            **arg_encoders
    ):
        """
        :param command_name: str - Name of the command within Libindy
        :param return_type: type - Custom C-return type
            ->  Can also be multiple types as tuple
        :param arg_encoders: callable - Custom argument encoding functions
            ->  The key must match the name of the argument it is encoding
        -----------------------------------------------------------------------
        :raises NotImplementedError: Libindy implements no command with the
            specified name
        """

        # Check if command is implemented in Libindy
        if not LIBINDY.implements_command(command_name):
            _msg = f'Command {command_name} is not implemented in Libindy!\n'
            _LOGGER.error(_msg)
            raise NotImplementedError(_msg)

        self._command_name: str = command_name
        self._encoders: Dict[str, Callable] = arg_encoders
        self._return_type: Union[type, Tuple, None] = return_type
        self._callback: CFUNCTYPE
        self._decoders: Union[Tuple, Callable]

    def __call__(
            self,
            command: callable
    ) -> Callable:
        """Builds the command function body.
        -----------------------------------------------------------------------
        This will read the command signature (arguments, return type etc.) of
        the passed command and will generate the function body according to
        those specifications. If the passed command function already has a
        body, it will be overwritten.
        -----------------------------------------------------------------------
        :param command: callable - Function signature to build body for
        -----------------------------------------------------------------------
        :returns final_command: callable - Final command function
        """
        _LOGGER.debug(f'Building {command.__qualname__}...')

        # Get argument and return type annotations
        full_arg_spec = inspect.getfullargspec(command)
        arg_names: list = full_arg_spec.args
        annotations: dict = inspect.getfullargspec(command).annotations

        # Set argument encoder functions
        if len(arg_names) > 0:
            self._set_argument_encoders(arg_names, annotations)
        else:
            _LOGGER.debug('  Command has no arguments; skipping.')

        # Set return type and response decoder functions
        return_type_tuple = ()
        if annotations.get('return'):

            # Map to tuple for streamlined processing
            return_type_tuple = annotations.get('return')
            if not isinstance(return_type_tuple, tuple):
                return_type_tuple = (return_type_tuple,)

            # Set default return types if no custom return types are specified
            if not self._return_type:
                self._set_return_types(return_type_tuple)
            else:
                _LOGGER.debug('  Command has custom return type(s); skipping.')

            # Set response decoder functions
            self._set_response_decoders(return_type_tuple)
        else:
            _LOGGER.debug('  Command returns nothing; skipping.')

        # Build callback function
        self._set_callback(return_type_tuple)

        @wraps(command)
        async def wrapped_command(*args, **kwargs) -> Any:
            starting_time = time.clock()

            # Check if argument requirements are satisfied
            _args, _kwargs, kwargs_strings = list(args), {}, []
            for arg_name in arg_names:
                if arg_name in kwargs.keys():
                    _kwargs[arg_name] = kwargs.get(arg_name)
                elif len(_args) > 0:
                    _kwargs[arg_name] = _args.pop(0)
                else:
                    msg = f"{command.__qualname__} missing 1 required " \
                        f"positional argument: '{arg_name}'\n"
                    _LOGGER.error(msg)
                    raise TypeError(msg)

                # Build key / value pair string
                kwargs_strings.append(
                    f'{arg_name}={str(_kwargs.get(arg_name))}'
                )

            # Build and log command entry string
            _LIBINDY_LOGGER.info(
                f'{command.__qualname__} >>> {", ".join(kwargs_strings)}'
            )

            # Encode arguments
            encoded_args = []
            for arg_name in arg_names:
                encoded_arg = self._encoders[arg_name](_kwargs.get(arg_name))

                if isinstance(encoded_arg, tuple):
                    encoded_args.extend(encoded_arg)
                else:
                    encoded_args.append(encoded_arg)

            # Run Libindy command
            response = await LIBINDY(self._command_name,
                                     *encoded_args,
                                     self._callback)

            # Decode response if necessary
            decoded_response = []
            if self._return_type:
                for element, decoder in zip(response, self._decoders):
                    decoded_response.append(decoder(element))

            # Map list to tuple, single element or None
            if not self._return_type:
                decoded_response = None
            elif len(self._return_type) > 1:
                decoded_response = tuple(decoded_response)
            else:
                decoded_response = decoded_response[0]

            _LIBINDY_LOGGER.info(
                f'{command.__qualname__} '
                f'[{(time.clock() - starting_time):.2f}s] <<< '
                f'{str(decoded_response)}'
            )

            return decoded_response

        # Apply command annotations and signature from old command to new one
        wrapped_command.__annotations__ = command.__annotations__
        wrapped_command.__signature__ = inspect.signature(command)
        _LOGGER.debug(f'Finished building {command.__qualname__}.')

        return wrapped_command

    # Builder Functions -------------------------------------------------------
    def _set_argument_encoders(
            self,
            arg_names: list,
            arg_signatures: dict
    ):
        """Sets the command's argument encoder functions.
        -----------------------------------------------------------------------
        This will assign functions that encode the argument value to their
        C-type equivalent to every argument, provided it has no custom encoding
        function specified in LibindyCommand.__init__.
        -----------------------------------------------------------------------
        :param arg_names: list - The names of the command arguments
        :param arg_signatures: dict - The type specs of the command arguments
        -----------------------------------------------------------------------
        :raises TypeError: An argument type has no default encoder function
        """
        _LOGGER.debug('  Setting argument encoders...')

        # Ignore arguments with custom encoders
        arg_names = list(filter(
            lambda arg: arg not in self._encoders.keys(), arg_names
        ))

        for name in arg_names:
            optional, arg_type = self._is_optional(arg_signatures[name])

            # Assign argument type encoding function
            if arg_type in {Union[dict, str], Union[list, str], str}:
                self._encoders[name] = _encode_str_or_collection
            elif arg_type is int:
                self._encoders[name] = _encode_int
            elif arg_type is bool:
                self._encoders[name] = _encode_bool
            elif arg_type is bytes:
                self._encoders[name] = _encode_bytes
            else:
                msg = f'Unsupported argument type {arg_type}!'
                _LOGGER.error(f'\n  {msg}')
                raise TypeError(msg)

            # Add "None" check before encoding if argument is optional
            if optional:
                self._encoders[name] = _run_optional(self._encoders[name])
        _LOGGER.debug('  Argument encoders set.')

    def _set_return_types(
            self,
            return_type_tuple: Tuple
    ):
        """Maps Python return type(s) to C-encoded return type(s).
        -----------------------------------------------------------------------
        :param return_type_tuple: tuple - Python return type(s) as tuple
        """
        _LOGGER.debug('  Setting return type(s)...')

        c_return_types = []
        for return_type in return_type_tuple:
            _, return_type = LibindyCommand._is_optional(return_type)

            c_return_type = _RETURN_TYPE_MAP.get(return_type)
            if not return_type:
                msg = f'Unsupported return type {return_type}!'
                _LOGGER.error(f'\n  {msg}')
                raise TypeError(msg)

            if isinstance(c_return_type, tuple):
                c_return_types.extend(c_return_type)
            else:
                c_return_types.append(c_return_type)

        self._return_type = tuple(c_return_types)
        _LOGGER.debug('  Return type(s) set.')

    def _set_response_decoders(
            self,
            return_type_tuple: Tuple
    ):
        """Maps matching decode function to Python return type.
        -----------------------------------------------------------------------
        :param return_type_tuple: tuple - Python return type(s) as tuple
        """
        _LOGGER.debug('  Setting response decoders...')

        decoders = []
        for return_type in return_type_tuple:
            optional, return_type = self._is_optional(return_type)

            if return_type in {dict, list}:
                decoders.append(_decode_collection)
            elif return_type is str:
                decoders.append(_decode_str)
            else:
                decoders.append(_decode_default)

            if optional:
                decoders[-1] = _run_optional(decoders[-1])

        self._decoders = tuple(decoders)
        _LOGGER.debug('  Response decoders set.')

    def _set_callback(
            self,
            return_type_tuple: Tuple
    ):
        """Builds a callback function that matches the function signature.
        -----------------------------------------------------------------------
        :param return_type_tuple: tuple - Python return type(s) as tuple
        """
        _LOGGER.debug('  Setting callback function...')

        callback_transform = None
        if bytes in return_type_tuple:

            # Libindy returns bytes in two parts that have to be combined
            # before giving back to the library caller. This is taken care of
            # with this function
            def callback_transform(*cb_args) -> Any:
                cb_args, tf_args = list(cb_args), []

                for index, return_type in enumerate(return_type_tuple):
                    if return_type is bytes:
                        # Create bytes object using the bytes value and the
                        # content length stored in the next list entry
                        tf_arg = bytes(cb_args[index][:cb_args.pop(index + 1)])
                        tf_args.append(tf_arg)
                    else:
                        tf_args.append(cb_args[index])

                return tuple(tf_args)

        return_types = self._return_type or ()
        signature = CFUNCTYPE(None, c_int32, c_int32, *return_types)
        self._callback = LIBINDY.create_callback(signature, callback_transform)
        _LOGGER.debug('  Callback function set.')

    # Helper Methods ----------------------------------------------------------
    @staticmethod
    def _is_optional(spec: Any) -> (bool, Union):
        """Determines whether a type is an Optional.
        -----------------------------------------------------------------------
        This will check if the provided type ( -> spec) is wrapped with a
        typing.Optional object. If so, the "None" will be returned from the
        Union. Afterwards (or if spec is not optional), it will return a bool
        saying whether spec is optional or not and a new Union with the
        remaining types.
        -----------------------------------------------------------------------
        :param spec: Union, type - Type specification to check
        -----------------------------------------------------------------------
        :returns: (
            is_optional: bool - Whether spec is optional or not
            types: Union, type - The remaining types after removing "None"
        )
        """

        # Check if spec is an Optional
        if str(spec).startswith(str(Union)) and type(None) in spec.__args__:
            types = spec.__args__
            types = tuple(filter(lambda t: not isinstance(None, t), types))
            return True, Union[types]
        return False, spec


# Argument Encoding Functions -------------------------------------------------
def _encode_str_or_collection(arg: Union[dict, list, str]) -> c_char_p:
    if isinstance(arg, (dict, list)):
        arg = json.dumps(arg)
    return c_char_p(arg.encode('utf-8'))


def _encode_int(arg: int) -> c_int32:
    return c_int32(arg)


def _encode_bool(arg: bool) -> c_bool:
    return c_bool(arg)


def _encode_bytes(arg: bytes) -> (bytes, c_uint32):
    return arg, c_uint32(len(arg))


# Response Decoding Functions -------------------------------------------------
def _decode_str(res: Any) -> str:
    return res.decode()


def _decode_collection(res: Any) -> Union[dict, list]:
    return json.loads(_decode_str(res))


def _decode_default(res: Any) -> Union[int, bool, bytes]:
    return res


# Helper Functions ------------------------------------------------------------
def _run_optional(func: Callable) -> Callable:
    return lambda arg: None if not arg else func(arg)


_RETURN_TYPE_MAP = {
    str: c_char_p,
    int: c_int32,
    bool: c_bool,
    bytes: (POINTER(c_uint8), c_uint32),
    dict: c_char_p,
    list: c_char_p
}
