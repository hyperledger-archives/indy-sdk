from .libindy import do_call, create_cb

from typing import Optional
from ctypes import *

import logging


async def add_wallet_record(wallet_handle: int,
                            type_: str,
                            id_: str,
                            value: str,
                            tags_json: Optional[str]) -> None:
    """
    Create a new non-secret record in the wallet

    :param wallet_handle: wallet handler (created by open_wallet).
    :param type_: allows to separate different record types collections
    :param id_: the id of record
    :param value: the value of record
    :param tags_json: the record tags used for search and storing meta information as json:
       {
         "tagName1": <str>, // string tag (will be stored encrypted)
         "tagName2": <str>, // string tag (will be stored encrypted)
         "~tagName3": <str>, // string tag (will be stored un-encrypted)
         "~tagName4": <str>, // string tag (will be stored un-encrypted)
       }
    :return: None
    """

    logger = logging.getLogger(__name__)
    logger.debug("add_wallet_record: >>> wallet_handle: %r, type_: %r, id: %r, value: %r, tags_json: %r",
                 wallet_handle,
                 type_,
                 id_,
                 value,
                 tags_json)

    if not hasattr(add_wallet_record, "cb"):
        logger.debug("add_wallet_record: Creating callback")
        add_wallet_record.cb = create_cb(CFUNCTYPE(None, c_int32, c_int32))

    c_wallet_handle = c_int32(wallet_handle)
    c_type = c_char_p(type_.encode('utf-8'))
    c_id = c_char_p(id_.encode('utf-8'))
    c_value = c_char_p(value.encode('utf-8'))
    c_tags_json = c_char_p(tags_json.encode('utf-8')) if tags_json is not None else None

    res = await do_call('indy_add_wallet_record',
                        c_wallet_handle,
                        c_type,
                        c_id,
                        c_value,
                        c_tags_json,
                        add_wallet_record.cb)

    logger.debug("add_wallet_record: <<< res: %r", res)
    return res


async def update_wallet_record_value(wallet_handle: int,
                                     type_: str,
                                     id_: str,
                                     value: str) -> None:
    """
    Update a non-secret wallet record value

    :param wallet_handle: wallet handler (created by open_wallet).
    :param type_: allows to separate different record types collections
    :param id_: the id of record
    :param value: the value of record
    :return: None
    """

    logger = logging.getLogger(__name__)
    logger.debug("update_wallet_record_value: >>> wallet_handle: %r, type_: %r, id: %r, value: %r",
                 wallet_handle,
                 type_,
                 id_,
                 value)

    if not hasattr(update_wallet_record_value, "cb"):
        logger.debug("update_wallet_record_value: Creating callback")
        update_wallet_record_value.cb = create_cb(CFUNCTYPE(None, c_int32, c_int32))

    c_wallet_handle = c_int32(wallet_handle)
    c_type = c_char_p(type_.encode('utf-8'))
    c_id = c_char_p(id_.encode('utf-8'))
    c_value = c_char_p(value.encode('utf-8'))

    res = await do_call('indy_update_wallet_record_value',
                        c_wallet_handle,
                        c_type,
                        c_id,
                        c_value,
                        update_wallet_record_value.cb)

    logger.debug("update_wallet_record_value: <<< res: %r", res)
    return res


async def update_wallet_record_tags(wallet_handle: int,
                                    type_: str,
                                    id_: str,
                                    tags_json: str) -> None:
    """
    Update a non-secret wallet record value

    :param wallet_handle: wallet handler (created by open_wallet).
    :param type_: allows to separate different record types collections
    :param id_: the id of record
    :param tags_json: ags_json: the record tags used for search and storing meta information as json:
       {
         "tagName1": <str>, // string tag (will be stored encrypted)
         "tagName2": <str>, // string tag (will be stored encrypted)
         "~tagName3": <str>, // string tag (will be stored un-encrypted)
         "~tagName4": <str>, // string tag (will be stored un-encrypted)
       }
    :return: None
    """

    logger = logging.getLogger(__name__)
    logger.debug("update_wallet_record_tags: >>> wallet_handle: %r, type_: %r, id: %r, tags_json: %r",
                 wallet_handle,
                 type_,
                 id_,
                 tags_json)

    if not hasattr(update_wallet_record_tags, "cb"):
        logger.debug("update_wallet_record_tags: Creating callback")
        update_wallet_record_tags.cb = create_cb(CFUNCTYPE(None, c_int32, c_int32))

    c_wallet_handle = c_int32(wallet_handle)
    c_type = c_char_p(type_.encode('utf-8'))
    c_id = c_char_p(id_.encode('utf-8'))
    c_tags_json = c_char_p(tags_json.encode('utf-8'))

    res = await do_call('indy_update_wallet_record_tags',
                        c_wallet_handle,
                        c_type,
                        c_id,
                        c_tags_json,
                        update_wallet_record_tags.cb)

    logger.debug("update_wallet_record_tags: <<< res: %r", res)
    return res


async def add_wallet_record_tags(wallet_handle: int,
                                 type_: str,
                                 id_: str,
                                 tags_json: str) -> None:
    """
    Add new tags to the wallet record

    :param wallet_handle: wallet handler (created by open_wallet).
    :param type_: allows to separate different record types collections
    :param id_: the id of record
    :param tags_json: ags_json: the record tags used for search and storing meta information as json:
       {
         "tagName1": <str>, // string tag (will be stored encrypted)
         "tagName2": <str>, // string tag (will be stored encrypted)
         "~tagName3": <str>, // string tag (will be stored un-encrypted)
         "~tagName4": <str>, // string tag (will be stored un-encrypted)
       }
    :return: None
    """

    logger = logging.getLogger(__name__)
    logger.debug("add_wallet_record_tags: >>> wallet_handle: %r, type_: %r, id: %r, tags_json: %r",
                 wallet_handle,
                 type_,
                 id_,
                 tags_json)

    if not hasattr(add_wallet_record_tags, "cb"):
        logger.debug("add_wallet_record_tags: Creating callback")
        add_wallet_record_tags.cb = create_cb(CFUNCTYPE(None, c_int32, c_int32))

    c_wallet_handle = c_int32(wallet_handle)
    c_type = c_char_p(type_.encode('utf-8'))
    c_id = c_char_p(id_.encode('utf-8'))
    c_tags_json = c_char_p(tags_json.encode('utf-8'))

    res = await do_call('indy_add_wallet_record_tags',
                        c_wallet_handle,
                        c_type,
                        c_id,
                        c_tags_json,
                        add_wallet_record_tags.cb)

    logger.debug("add_wallet_record_tags: <<< res: %r", res)
    return res


async def delete_wallet_record_tags(wallet_handle: int,
                                    type_: str,
                                    id_: str,
                                    tag_names_json: str) -> None:
    """
    Add new tags to the wallet record

    :param wallet_handle: wallet handler (created by open_wallet).
    :param type_: allows to separate different record types collections
    :param id_: the id of record
    :param tag_names_json: the list of tag names to remove from the record as json array: ["tagName1", "tagName2", ...]
    :return: None
    """

    logger = logging.getLogger(__name__)
    logger.debug("delete_wallet_record_tags: >>> wallet_handle: %r, type_: %r, id: %r, tag_names_json: %r",
                 wallet_handle,
                 type_,
                 id_,
                 tag_names_json)

    if not hasattr(delete_wallet_record_tags, "cb"):
        logger.debug("delete_wallet_record_tags: Creating callback")
        delete_wallet_record_tags.cb = create_cb(CFUNCTYPE(None, c_int32, c_int32))

    c_wallet_handle = c_int32(wallet_handle)
    c_type = c_char_p(type_.encode('utf-8'))
    c_id = c_char_p(id_.encode('utf-8'))
    c_tag_names_json = c_char_p(tag_names_json.encode('utf-8'))

    res = await do_call('indy_delete_wallet_record_tags',
                        c_wallet_handle,
                        c_type,
                        c_id,
                        c_tag_names_json,
                        delete_wallet_record_tags.cb)

    logger.debug("delete_wallet_record_tags: <<< res: %r", res)
    return res


async def delete_wallet_record(wallet_handle: int,
                               type_: str,
                               id_: str) -> None:
    """
    Delete an existing wallet record in the wallet

    :param wallet_handle: wallet handler (created by open_wallet).
    :param type_: allows to separate different record types collections
    :param id_: the id of record
    :return: None
    """

    logger = logging.getLogger(__name__)
    logger.debug("delete_wallet_record: >>> wallet_handle: %r, type_: %r, id: %r",
                 wallet_handle,
                 type_,
                 id_)

    if not hasattr(delete_wallet_record, "cb"):
        logger.debug("delete_wallet_record: Creating callback")
        delete_wallet_record.cb = create_cb(CFUNCTYPE(None, c_int32, c_int32))

    c_wallet_handle = c_int32(wallet_handle)
    c_type = c_char_p(type_.encode('utf-8'))
    c_id = c_char_p(id_.encode('utf-8'))

    res = await do_call('indy_delete_wallet_record',
                        c_wallet_handle,
                        c_type,
                        c_id,
                        delete_wallet_record.cb)

    logger.debug("delete_wallet_record: <<< res: %r", res)
    return res


async def get_wallet_record(wallet_handle: int,
                            type_: str,
                            id: str,
                            options_json: str) -> str:
    """
    Get an wallet record by id

    :param wallet_handle: wallet handler (created by open_wallet).
    :param type_: allows to separate different record types collections
    :param id: the id of record
    :param options_json: //TODO: FIXME: Think about replacing by bitmask
      {
        retrieveType: (optional, false by default) Retrieve record type,
        retrieveValue: (optional, true by default) Retrieve record value,
        retrieveTags: (optional, true by default) Retrieve record tags
      }
    :return: wallet record json:
     {
       id: "Some id",
       type: "Some type", // present only if retrieveType set to true
       value: "Some value", // present only if retrieveValue set to true
       tags: <tags json>, // present only if retrieveTags set to true
     }
    """

    logger = logging.getLogger(__name__)
    logger.debug("get_wallet_record: >>> wallet_handle: %r, type_: %r, id: %r, options_json: %r",
                 wallet_handle,
                 type_,
                 id,
                 options_json)

    if not hasattr(get_wallet_record, "cb"):
        logger.debug("get_wallet_record: Creating callback")
        get_wallet_record.cb = create_cb(CFUNCTYPE(None, c_int32, c_int32, c_char_p))

    c_wallet_handle = c_int32(wallet_handle)
    c_type = c_char_p(type_.encode('utf-8'))
    c_id = c_char_p(id.encode('utf-8'))
    c_options_json = c_char_p(options_json.encode('utf-8'))

    wallet_record = await do_call('indy_get_wallet_record',
                                  c_wallet_handle,
                                  c_type,
                                  c_id,
                                  c_options_json,
                                  get_wallet_record.cb)
    res = wallet_record.decode()

    logger.debug("get_wallet_record: <<< res: %r", res)
    return res


async def open_wallet_search(wallet_handle: int,
                             type_: str,
                             query_json: str,
                             options_json: str) -> int:
    """
    Search for wallet records

    :param wallet_handle: wallet handler (created by open_wallet).
    :param type_: allows to separate different record types collections
    :param query_json: MongoDB style query to wallet record tags:
      {
        "tagName": "tagValue",
        $or: {
          "tagName2": { $regex: 'pattern' },
          "tagName3": { $gte: '123' },
        },
      }
    :param options_json: //TODO: FIXME: Think about replacing by bitmask
      {
        retrieveRecords: (optional, true by default) If false only "counts" will be calculated,
        retrieveTotalCount: (optional, false by default) Calculate total count,
        retrieveType: (optional, false by default) Retrieve record type,
        retrieveValue: (optional, true by default) Retrieve record value,
        retrieveTags: (optional, true by default) Retrieve record tags,
      }
    :return: search_handle: Wallet search handle that can be used later
             to fetch records by small batches (with fetch_wallet_search_next_records)
    """

    logger = logging.getLogger(__name__)
    logger.debug("open_wallet_search: >>> wallet_handle: %r, type_: %r, query_json: %r, options_json: %r",
                 wallet_handle,
                 type_,
                 query_json,
                 options_json)

    if not hasattr(open_wallet_search, "cb"):
        logger.debug("open_wallet_search: Creating callback")
        open_wallet_search.cb = create_cb(CFUNCTYPE(None, c_int32, c_int32, c_int32))

    c_wallet_handle = c_int32(wallet_handle)
    c_type = c_char_p(type_.encode('utf-8'))
    c_query_json = c_char_p(query_json.encode('utf-8'))
    c_options_json = c_char_p(options_json.encode('utf-8'))

    search_handle = await do_call('indy_open_wallet_search',
                                  c_wallet_handle,
                                  c_type,
                                  c_query_json,
                                  c_options_json,
                                  open_wallet_search.cb)
    res = search_handle

    logger.debug("open_wallet_search: <<< res: %r", res)
    return res


async def fetch_wallet_search_next_records(wallet_handle: int,
                                           wallet_search_handle: int,
                                           count: int) -> str:
    """
    Fetch next records for wallet search.

    :param wallet_handle: wallet handler (created by open_wallet).
    :param wallet_search_handle: wallet wallet handle (created by open_wallet_search)
    :param count: Count of records to fetch
    :return: wallet records json:
     {
       totalCount: <str>, // present only if retrieveTotalCount set to true
       records: [{ // present only if retrieveRecords set to true
           id: "Some id",
           type: "Some type", // present only if retrieveType set to true
           value: "Some value", // present only if retrieveValue set to true
           tags: <tags json>, // present only if retrieveTags set to true
       }],
     }
    """

    logger = logging.getLogger(__name__)
    logger.debug("fetch_wallet_search_next_records: >>> wallet_handle: %r, wallet_search_handle: %r, count: %r",
                 wallet_handle,
                 wallet_search_handle,
                 count)

    if not hasattr(fetch_wallet_search_next_records, "cb"):
        logger.debug("fetch_wallet_search_next_records: Creating callback")
        fetch_wallet_search_next_records.cb = create_cb(CFUNCTYPE(None, c_int32, c_int32, c_char_p))

    c_wallet_handle = c_int32(wallet_handle)
    c_wallet_search_handle = c_int32(wallet_search_handle)
    c_count = c_uint(count)

    records_json = await do_call('indy_fetch_wallet_search_next_records',
                                 c_wallet_handle,
                                 c_wallet_search_handle,
                                 c_count,
                                 fetch_wallet_search_next_records.cb)
    res = records_json.decode()

    logger.debug("fetch_wallet_search_next_records: <<< res: %r", res)
    return res


async def close_wallet_search(wallet_search_handle: int) -> None:
    """
    Close wallet search (make search handle invalid)

    :param wallet_search_handle: wallet wallet handle (created by open_wallet_search)
    :return: None
    """

    logger = logging.getLogger(__name__)
    logger.debug("close_wallet_search: >>> wallet_search_handle: %r",
                 wallet_search_handle)

    if not hasattr(close_wallet_search, "cb"):
        logger.debug("close_wallet_search: Creating callback")
        close_wallet_search.cb = create_cb(CFUNCTYPE(None, c_int32, c_int32))

    c_wallet_search_handle = c_int32(wallet_search_handle)

    res = await do_call('indy_close_wallet_search',
                        c_wallet_search_handle,
                        close_wallet_search.cb)

    logger.debug("close_wallet_search: <<< res: %r", res)
    return res
