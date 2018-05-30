import pytest
from vcx.error import VcxError
from vcx.api.wallet import *

TYPE = "TestType"
EMPTY_TYPE = ""
ID = "RecordId"
EMPTY_ID = ""
VALUE = "RecordValue"
VALUE_NEW = "RecordValueNew"
EMPTY_VALUE = ""
TAGS = "{\"tagName1\":\"str1\",\"tagName2\":\"5\",\"tagName3\":\"12\"}"
TAGS_EMPTY = ""
TAGS_EMPTY_JSON = "{}"
TAGS_MALFORMED_JSON = "{\"e\":}"
QUERY_JSON = {"tagName1": "str1"}
SEARCHED_RECORD = {
  "id": "RecordId",
  "type": None,
  "value": "RecordValue",
  "tags": TAGS
}


@pytest.mark.asyncio
@pytest.mark.usefixtures('vcx_init_test_mode')
async def test_get_token_info():
    info = await Wallet.get_token_info(0)
    assert info


@pytest.mark.asyncio
@pytest.mark.usefixtures('vcx_init_test_mode')
async def test_send_tokens():
    receipt = await Wallet.send_tokens(0,50.0,"address")
    assert receipt

@pytest.mark.asyncio
@pytest.mark.usefixtures('vcx_init_test_mode')
async def test_create_payment_address():
    address = await Wallet.create_payment_address()
    assert address

@pytest.mark.asyncio
async def test_wallet_storage():
    await Wallet.add_record(TYPE, ID, VALUE, TAGS)
    with pytest.raises(VcxError) as e:
        await Wallet.add_record(EMPTY_TYPE, ID, VALUE, TAGS)
    with pytest.raises(VcxError) as e:
        await Wallet.add_record(TYPE, EMPTY_ID, VALUE, TAGS)
    with pytest.raises(VcxError) as e:
        await Wallet.add_record(TYPE, ID, EMPTY_VALUE, TAGS)
    with pytest.raises(VcxError) as e:
        await Wallet.add_record(TYPE, ID, VALUE, TAGS_EMPTY)
    with pytest.raises(VcxError) as e:
        await Wallet.add_record(TYPE, ID, VALUE, TAGS_MALFORMED_JSON)

    await Wallet.update_record_value(TYPE, ID, VALUE_NEW)
    await Wallet.update_record_tags(TYPE, ID, TAGS_EMPTY_JSON)
    await Wallet.add_record_tags(TYPE, ID, TAGS)
    await Wallet.delete_record_tags(TYPE, ID, ['one', 'two'])
    await Wallet.delete_record(TYPE, ID)
    record = {
        "id": ID,
        "type": TYPE,
        "value": VALUE,
        "tags": None,
    }
    assert (json.loads(await Wallet.get_record(TYPE, ID)) == record)
    search_handle = await Wallet.open_search(TYPE, QUERY_JSON, None)
    assert(search_handle == 1)
    searched_record = await Wallet.search_next_records(search_handle, 1)
    assert(json.loads(searched_record) == SEARCHED_RECORD)
    await Wallet.close_search(search_handle)
