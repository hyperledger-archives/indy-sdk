import json

from indy import non_secrets

type_ = "TestType"
id1 = "RecordId"
id2 = "RecordId2"
id3 = "RecordId3"
value1 = "RecordValue"
value2 = "RecordValue2"
value3 = "RecordValue3"
tags_empty = "{}"
options_empty = "{}"
query_empty = "{}"
tags1 = '{"tagName1":"str1","tagName2":"5","tagName3":"12"}'
tags2 = '{"tagName1":"str2","tagName2":"pre_str3","tagName3":"2"}'
tags3 = '{"tagName1":"str1","tagName2":"str2","tagName3":"str3"}'


async def check_record_field(wallet_handle: int, field: str, expected_value: str):
    record = json.loads(
        await non_secrets.get_wallet_record(wallet_handle, id1, type_, options_empty))

    if field == 'value':
        assert expected_value == record['value']
    elif field == 'tags':
        assert json.loads(expected_value) == record['tags']
    else:
        assert False
