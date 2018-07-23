from ctypes import *
from vcx.common import do_call, create_cb
import json

import logging


class Wallet:

    @staticmethod
    async def close_search(handle: int):
        logger = logging.getLogger(__name__)

        if not hasattr(Wallet.close_search, "cb"):
            logger.debug("vcx_wallet_close_search: Creating callback")
            Wallet.close_search.cb = create_cb(CFUNCTYPE(None, c_uint32, c_uint32))

        c_handle = c_uint32(handle)

        data = await do_call('vcx_wallet_close_search',
                             c_handle,
                             Wallet.close_search.cb)

        logger.debug("vcx_wallet_close_search completed")
        return data

    @staticmethod
    async def open_search(type_: str, query: dict, options: dict):
        logger = logging.getLogger(__name__)

        if not hasattr(Wallet.open_search, "cb"):
            logger.debug("vcx_wallet_open_search: Creating callback")
            Wallet.open_search.cb = create_cb(CFUNCTYPE(None, c_uint32, c_uint32, c_uint32))

        c_type_ = c_char_p(type_.encode('utf-8'))
        c_query = c_char_p(json.dumps(query).encode('utf-8'))
        c_options = c_char_p(json.dumps(options).encode('utf-8')) if options else None

        data = await do_call('vcx_wallet_open_search',
                             c_type_,
                             c_query,
                             c_options,
                             Wallet.open_search.cb)

        logger.debug("vcx_wallet_open_search completed")
        return data

    @staticmethod
    async def search_next_records(handle: int, count: int):
        logger = logging.getLogger(__name__)

        if not hasattr(Wallet.search_next_records, "cb"):
            logger.debug("vcx_wallet_search_next_records: Creating callback")
            Wallet.search_next_records.cb = create_cb(CFUNCTYPE(None, c_uint32, c_uint32, c_char_p))
        c_handle = c_uint32(handle)
        c_count = c_uint32(count)

        data = await do_call('vcx_wallet_search_next_records',
                             c_handle,
                             c_count,
                             Wallet.search_next_records.cb)

        logger.debug("vcx_wallet_search_next_records completed")
        return data.decode()

    @staticmethod
    async def get_record(type_: str, id: str, options: str):
        logger = logging.getLogger(__name__)

        if not hasattr(Wallet.get_record, "cb"):
            logger.debug("vcx_wallet_get_record: Creating callback")
            Wallet.get_record.cb = create_cb(CFUNCTYPE(None, c_uint32, c_uint32, c_char_p))

        c_type_ = c_char_p(type_.encode('utf-8'))
        c_id = c_char_p(id.encode('utf-8'))
        c_options = c_char_p(options.encode('utf-8'))
        data = await do_call('vcx_wallet_get_record',
                             c_type_,
                             c_id,
                             c_options,
                             Wallet.get_record.cb)

        logger.debug("vcx_wallet_get_record completed")
        return data.decode()

    @staticmethod
    async def delete_record(type_: str, id: str):
        logger = logging.getLogger(__name__)

        if not hasattr(Wallet.delete_record, "cb"):
            logger.debug("vcx_wallet_delete_record: Creating callback")
            Wallet.delete_record.cb = create_cb(CFUNCTYPE(None, c_uint32, c_uint32))

        c_type_ = c_char_p(type_.encode('utf-8'))
        c_id = c_char_p(id.encode('utf-8'))
        result = await do_call('vcx_wallet_delete_record',
                               c_type_,
                               c_id,
                               Wallet.delete_record.cb)

        logger.debug("vcx_wallet_delete_record completed")
        return result

    @staticmethod
    async def delete_record_tags(type_: str, id: str, tags: list ):
        logger = logging.getLogger(__name__)

        if not hasattr(Wallet.delete_record_tags, "cb"):
            logger.debug("vcx_wallet_delete_record_tags: Creating callback")
            Wallet.delete_record_tags.cb = create_cb(CFUNCTYPE(None, c_uint32, c_uint32))

        tags = json.dumps(tags)
        c_type_ = c_char_p(type_.encode('utf-8'))
        c_id = c_char_p(id.encode('utf-8'))
        c_tags = c_char_p(tags.encode('utf-8'))
        result = await do_call('vcx_wallet_delete_record_tags',
                               c_type_,
                               c_id,
                               c_tags,
                               Wallet.delete_record_tags.cb)

        logger.debug("vcx_wallet_delete_record_tags completed")
        return result

    @staticmethod
    async def add_record_tags(type_: str, id: str, tags: str):
        logger = logging.getLogger(__name__)

        if not hasattr(Wallet.add_record_tags, "cb"):
            logger.debug("vcx_wallet_add_record_tags: Creating callback")
            Wallet.add_record_tags.cb = create_cb(CFUNCTYPE(None, c_uint32, c_uint32))

        c_type_ = c_char_p(type_.encode('utf-8'))
        c_id = c_char_p(id.encode('utf-8'))
        c_tags = c_char_p(tags.encode('utf-8'))
        result = await do_call('vcx_wallet_add_record_tags',
                               c_type_,
                               c_id,
                               c_tags,
                               Wallet.add_record_tags.cb)

        logger.debug("vcx_wallet_add_record_tags completed")
        return result

    @staticmethod
    async def update_record_tags(type_: str, id: str, tags: str):
        logger = logging.getLogger(__name__)

        if not hasattr(Wallet.update_record_tags, "cb"):
            logger.debug("vcx_wallet_update_record_tags: Creating callback")
            Wallet.update_record_tags.cb = create_cb(CFUNCTYPE(None, c_uint32, c_uint32))

        c_type_ = c_char_p(type_.encode('utf-8'))
        c_id = c_char_p(id.encode('utf-8'))
        c_tags = c_char_p(tags.encode('utf-8'))
        result = await do_call('vcx_wallet_update_record_tags',
                               c_type_,
                               c_id,
                               c_tags,
                               Wallet.update_record_tags.cb)

        logger.debug("vcx_wallet_update_record_tags completed")
        return result

    @staticmethod
    async def update_record_value(type_: str, id: str, value: str):
        logger = logging.getLogger(__name__)

        if not hasattr(Wallet.update_record_value, "cb"):
            logger.debug("vcx_wallet_update_record_value: Creating callback")
            Wallet.update_record_value.cb = create_cb(CFUNCTYPE(None, c_uint32, c_uint32))

        c_type_ = c_char_p(type_.encode('utf-8'))
        c_id = c_char_p(id.encode('utf-8'))
        c_value = c_char_p(value.encode('utf-8'))
        result = await do_call('vcx_wallet_update_record_value',
                               c_type_,
                               c_id,
                               c_value,
                               Wallet.update_record_value.cb)

        logger.debug("vcx_wallet_update_record_value completed")
        return result

    @staticmethod
    async def add_record(type_: str, id: str, value: str, tags: str):
        logger = logging.getLogger(__name__)

        if not hasattr(Wallet.add_record, "cb"):
            logger.debug("vcx_wallet_add_record: Creating callback")
            Wallet.add_record.cb = create_cb(CFUNCTYPE(None, c_uint32, c_uint32))

        c_type_ = c_char_p(type_.encode('utf-8'))
        c_id = c_char_p(id.encode('utf-8'))
        c_value = c_char_p(value.encode('utf-8'))
        c_tags = c_char_p(tags.encode('utf-8'))
        result = await do_call('vcx_wallet_add_record',
                               c_type_,
                               c_id,
                               c_value,
                               c_tags,
                               Wallet.add_record.cb)

        logger.debug("vcx_wallet_add_record completed")
        return result

    @staticmethod
    async def get_token_info(handle: int) -> str:
        logger = logging.getLogger(__name__)

        if not hasattr(Wallet.get_token_info, "cb"):
            logger.debug("vcx_wallet_get_token_info: Creating callback")
            Wallet.get_token_info.cb = create_cb(CFUNCTYPE(None, c_uint32, c_uint32, c_char_p))

        c_payment = c_uint32(handle)

        result = await do_call('vcx_wallet_get_token_info',
                               c_payment,
                               Wallet.get_token_info.cb)

        logger.debug("vcx_wallet_get_token_info completed")
        return result

    @staticmethod
    async def create_payment_address(seed: str = None) -> str:
        logger = logging.getLogger(__name__)

        if not hasattr(Wallet.create_payment_address, "cb"):
            logger.debug("vcx_wallet_create_payment_address: Creating callback")
            Wallet.create_payment_address.cb = create_cb(CFUNCTYPE(None, c_uint32, c_uint32, c_char_p))

        if seed:
            c_seed = c_char_p(seed.encode('utf-8'))
        else:
            c_seed = None

        result = await do_call('vcx_wallet_create_payment_address',
                               c_seed,
                               Wallet.create_payment_address.cb)

        logger.debug("vcx_wallet_create_payment_address completed")
        return result

    @staticmethod
    async def validate_payment_address(address: str) -> None:
        logger = logging.getLogger(__name__)

        if not hasattr(Wallet.validate_payment_address, "cb"):
            logger.debug("vcx_wallet_validate_payment_address: Creating callback")
            Wallet.validate_payment_address.cb = create_cb(CFUNCTYPE(None, c_uint32, c_uint32))

        c_address = c_char_p(address.encode('utf-8'))
        result = await do_call('vcx_wallet_validate_payment_address',
                               c_address,
                               Wallet.validate_payment_address.cb)

        logger.debug("vcx_wallet_validate_payment_address completed")
        return result

    @staticmethod
    async def send_tokens(payment_handle: int, tokens: int, address: str) -> str:
        logger = logging.getLogger(__name__)

        if not hasattr(Wallet.send_tokens, "cb"):
            logger.debug("vcx_wallet_send_tokens: Creating callback")
            Wallet.send_tokens.cb = create_cb(CFUNCTYPE(None, c_uint32, c_uint32, c_char_p))

        c_payment_handle = c_uint32(payment_handle)
        c_tokens = c_uint64(tokens)
        c_address = c_char_p(address.encode('utf-8'))

        result = await do_call('vcx_wallet_send_tokens',
                               c_payment_handle,
                               c_tokens,
                               c_address,
                               Wallet.send_tokens.cb)

        logger.debug("vcx_wallet_send_tokens completed")
        return result

    @staticmethod
    async def export(path, backup_key):
        """
        Exports opened wallet
        :param path: Path to export wallet to User's File System.
        :param backupKey: String representing the User's Key for securing (encrypting) the exported Wallet.
        :return:
        Error code - success indicates that the wallet was successfully exported.
        """
        logger = logging.getLogger(__name__)

        if not hasattr(Wallet.export, "cb"):
            logger.debug("vcx_wallet_export: Creating callback")
            Wallet.export.cb = create_cb(CFUNCTYPE(None, c_uint32, c_uint32))

        c_backupKey = c_char_p(backup_key.encode('utf-8'))
        c_path = c_char_p(path.encode('utf-8'))

        result = await do_call('vcx_wallet_export',
                               c_path,
                               c_backupKey,
                               Wallet.export.cb)

        logger.debug("vcx_wallet_export completed")
        return result

    @staticmethod
    async def import_wallet(config):
        """
        Imports wallet from file with given key.
        Cannot be used if wallet is already opened (Especially if vcx_init has already been used).
        :param config: Can be same config that is passed to vcx_init.
        Must include: '{"wallet_name":"","wallet_key":"","exported_wallet_path":"","backup_key":""}'
        :return:
        Error code - success indicates that the wallet was successfully imported.
        """

        logger = logging.getLogger(__name__)

        if not hasattr(Wallet.import_wallet, "cb"):
            logger.debug("vcx_wallet_import: Creating callback")
            Wallet.import_wallet.cb = create_cb(CFUNCTYPE(None, c_uint32, c_uint32))

        c_config = c_char_p(config.encode('utf-8'))

        result = await do_call('vcx_wallet_import',
                               c_config,
                               Wallet.import_wallet.cb)

        logger.debug("vcx_wallet_export completed")
        return result

