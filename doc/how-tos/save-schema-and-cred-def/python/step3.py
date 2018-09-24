        # 9.
        print_log('\n9. Build the SCHEMA request to add new schema to the ledger as a Steward\n')
        seq_no = 1
        schema = {
            'seqNo': seq_no,
            'dest': steward_did,
            'data': {
                'id': '1',
                'name': 'gvt',
                'version': '1.0',
                'ver': '1.0',
                'attrNames': ['age', 'sex', 'height', 'name']
            }
        }
        schema_data = schema['data']
        print_log('Schema data: ')
        pprint.pprint(schema_data)
        print_log('Schema: ')
        pprint.pprint(schema)
        schema_request = await ledger.build_schema_request(steward_did, json.dumps(schema_data))
        print_log('Schema request: ')
        pprint.pprint(json.loads(schema_request))

        # 10.
        print_log('\n10. Sending the SCHEMA request to the ledger\n')
        schema_response = await ledger.sign_and_submit_request(pool_handle, wallet_handle, steward_did, schema_request)
        print_log('Schema response:')
        pprint.pprint(json.loads(schema_response))
