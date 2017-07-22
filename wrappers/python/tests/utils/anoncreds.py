from indy import anoncreds

import json

ISSUER_DID = "NcYxiDXkpYi6ov5FcYDi1e"
COMMON_SCHEMA_SEQ_NO = 1
COMMON_MASTER_SECRET_NAME = "common_master_secret_name"


async def prepare_common_wallet(wallet_handle):
    schema = get_gvt_schema_json(1)
    claim_def_json = await anoncreds.issuer_create_and_store_claim_def(
        wallet_handle, ISSUER_DID, json.dumps(schema), None, False)

    claim_offer_json_1 = get_claim_offer(ISSUER_DID, 1)
    claim_offer_json_2 = get_claim_offer(ISSUER_DID, 2)
    claim_offer_json_3 = get_claim_offer("CnEDk9HrMnmiHXEV1WFgbVCRteYnPqsJwrTdcZaNhFVW", 2)
    await anoncreds.prover_store_claim_offer(wallet_handle, json.dumps(claim_offer_json_1))
    await anoncreds.prover_store_claim_offer(wallet_handle, json.dumps(claim_offer_json_2))
    await anoncreds.prover_store_claim_offer(wallet_handle, json.dumps(claim_offer_json_3))
    await anoncreds.prover_create_master_secret(wallet_handle, COMMON_MASTER_SECRET_NAME)

    claim_req = await anoncreds.prover_create_and_store_claim_req(
        wallet_handle, "HEJ9gvWX64wW7UD", json.dumps(claim_offer_json_1), claim_def_json, COMMON_MASTER_SECRET_NAME)
    (_, claim_json) = await anoncreds.issuer_create_claim(
        wallet_handle, claim_req, json.dumps(get_gvt_claim_json()), -1, -1)
    await anoncreds.prover_store_claim(wallet_handle, claim_json)


def get_claim_offer(issuer_did, schema_seq_no):
    return {"issuer_did": issuer_did, "schema_seq_no": schema_seq_no}


def get_gvt_schema_json(schema_seq_no: int):
    return {
        "seqNo": schema_seq_no,
        "data": {
            "name": "gvt",
            "version": "1.0",
            "keys": ["age", "sex", "height", "name"]
        }
    }


def get_gvt_claim_json():
    return {
        "sex": ["male", "5944657099558967239210949258394887428692050081607692519917050011144233115103"],
        "name": ["Alex", "1139481716457488690172217916278103335"],
        "height": ["175", "175"],
        "age": ["28", "28"]
    }


def get_claim_req():
    return {"blinded_ms": {"prover_did": "CnEDk9HrMnmiHXEV1WFgbVCRteYnPqsJwrTdcZaNhFVW",
                           "u": "541727375645293327107242131390489410830131768916446771173223218236303087346206273292"
                                "275918450941006362568297619591573147842939390451766213271549909084590728218268187187"
                                "396963232997879281735355290245565403237095788507069932942349664408266908992668726827"
                                "902285139739992123705745482398771085112836294238073386324354310973398756650754537851"
                                "417229890983878959703959824327090115058645337274155525667150696753462207525844495604"
                                "072614465677317118141888367033373659867254296561952756168465435357073642154989807508"
                                "60746440672050640048215761507774996460985293327604627646056062013419674090094698841"
                                "792968543317468164175921100038",
                           "ur": None}, "issuer_did": ISSUER_DID, "schema_seq_no": 1}
