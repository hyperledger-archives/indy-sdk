from ctypes import *
import logging
from vcx.common import do_call, do_call_sync, create_cb

__all__ = ["vcx_init", "vcx_init_with_config"]

async def vcx_init(config_path: str) -> None:
    """
    Initializes VCX with config file.
    The list of available options see here: https://github.com/hyperledger/indy-sdk/blob/master/docs/configuration.md

    :param config_path: String - path to a config file to populate config attributes
    Example:
    await vcx_init('/home/username/vcxconfig.json')
    :return:
    """
    logger = logging.getLogger(__name__)

    if not hasattr(vcx_init, "cb"):
        logger.debug("vcx_init: Creating callback")
        vcx_init.cb = create_cb(CFUNCTYPE(None, c_uint32, c_uint32))

    c_config_path = c_char_p(config_path.encode('utf-8'))

    result = await do_call('vcx_init',
                           c_config_path,
                           vcx_init.cb)

    logger.debug("vcx_init completed")
    return result


async def vcx_init_with_config(config: str) -> None:
    """
    Initializes VCX with config settings

    :param config: config as json.
    The list of available options see here: https://github.com/hyperledger/indy-sdk/blob/master/docs/configuration.md

    Example:
    config = {
      "agency_did": "VsKV7grR1BUE29mG2Fm2kX",
      "agency_verkey": "Hezce2UWMZ3wUhVkh2LfKSs8nDzWwzs2Win7EzNN3YaR",
      "agency_endpoint": "http://localhost:8080",
      "genesis_path":"/var/lib/indy/verity-staging/pool_transactions_genesis",
      "institution_name": "institution",
      "institution_logo_url": "http://robohash.org/234",
      "institution_did": "EwsFhWVoc3Fwqzrwe998aQ",
      "institution_verkey": "8brs38hPDkw5yhtzyk2tz7zkp8ijTyWnER165zDQbpK6",
      "remote_to_sdk_did": "EtfeMFytvYTKnWwqTScp9D",
      "remote_to_sdk_verkey": "8a7hZDyJK1nNCizRCKMr4H4QbDm8Gg2vcbDRab8SVfsi",
      "sdk_to_remote_did": "KacwZ2ndG6396KXJ9NDDw6",
      "sdk_to_remote_verkey": "B8LgZGxEPcpTJfZkeqXuKNLihM1Awm8yidqsNwYi5QGc"
    }
    await vcx_init_with_config(config)
    :return:
    """
    logger = logging.getLogger(__name__)

    if not hasattr(vcx_init_with_config, "cb"):
        logger.debug("vcx_init_with_config: Creating callback")
        vcx_init_with_config.cb = create_cb(CFUNCTYPE(None, c_uint32, c_uint32))

    c_config = c_char_p(config.encode('utf-8'))

    result = await do_call('vcx_init_with_config',
                           c_config,
                           vcx_init_with_config.cb)

    logger.debug("vcx_init_with_config completed")
    return result

def vcx_init_minimal(config_string: str) -> None:
    """
    Initializes VCX with minimal (no-agency) config file AFTER the wallet and pool are set.

    :param config_string: String
    Example:
    vcx_wallet_set_handle(wallet_handle)
    vcx_pool_set_handle(pool_handle)
    await vcx_init_minimal('{"wallet_name":"wallet1",.....}')
    :return:
    """
    logger = logging.getLogger(__name__)

    c_config_string = c_char_p(config_string.encode('utf-8'))

    result = do_call_sync('vcx_init_minimal', c_config_string)

    logger.debug("vcx_init_minimal completed")
    return result