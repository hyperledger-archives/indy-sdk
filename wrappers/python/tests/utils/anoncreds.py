ISSUER_DID = "NcYxiDXkpYi6ov5FcYDi1e"


def get_gvt_schema_json(schema_seq_no: int):
    return {
        "seqNo": schema_seq_no,
        "data": {
            "name": "gvt",
            "version": "1.0",
            "keys": ["age", "sex", "height", "name"]
        }
    }
