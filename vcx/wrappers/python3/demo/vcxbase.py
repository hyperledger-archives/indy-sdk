import asyncio
from vcx.api.schema import Schema
from vcx.api.vcx_init import vcx_init


class VCXBase:

    @staticmethod
    def get_loop():
        if not hasattr(VCXBase.get_loop, "loop"):
            VCXBase.get_loop.loop = asyncio.get_event_loop()
        return VCXBase.get_loop.loop

    @staticmethod
    def lookup_schema(source_id, sequence_number):
        res = VCXBase.get_loop().run_until_complete(asyncio.gather(Schema.lookup(source_id, sequence_number)))
        if len(res) > 0:
            return res[0]

    @staticmethod
    def _do_async_method(fn, kwargs):
        res = VCXBase.get_loop().run_until_complete(asyncio.gather(fn(**kwargs)))
        if len(res) > 0:
            return res[0]

    @staticmethod
    async def _create_schema(source_id: str, name: str, attr: dict):
        return await Schema.create(source_id, name, attr)

    @staticmethod
    def _create_attr(attr: str, did: str, schema_seq_number: int):
        return {'schema_seq_no': schema_seq_number, 'name': attr, 'issuer_did': did}

    @staticmethod
    def format_proof_attrs(did: str, schema_seq_number: int, attr_list: list):
        formatted_attrs = []
        for a in attr_list:
            formatted_attrs.append(VCXBase._create_attr(a, did, schema_seq_number))

        return formatted_attrs

    @staticmethod
    async def _serialize_schema(schema):
        return await schema.serialize()

    @staticmethod
    def init(path):
        results = VCXBase.get_loop().run_until_complete(asyncio.gather(vcx_init(path)))
        print(results)
        return 1