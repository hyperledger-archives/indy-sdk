        # Here we are creating a third DID. This one is never written to the ledger, but we do have to have it in the
        # wallet, because every request to the ledger has to be signed by some requester. By creating a DID here, we
        # are forcing the wallet to allocate a keypair and identity that we can use to sign the request that's going
        # to read the trust anchor's info from the ledger.
        print_log('\n9. Generating and storing DID and verkey representing a Client '
                  'that wants to obtain Trust Anchor Verkey\n')
        client_did, client_verkey = await signus.create_and_store_my_did(wallet_handle, "{}")
        print_log('Client DID: ', client_did)
        print_log('Client Verkey: ', client_verkey)

        print_log('\n10. Building the GET_NYM request to query trust anchor verkey\n')
        get_nym_request = await ledger.build_get_nym_request(submitter_did=client_did,
                                                             target_did=trust_anchor_did)
        print_log('GET_NYM request: ')
        pprint.pprint(json.loads(get_nym_request))

        print_log('\n11. Sending the Get NYM request to the ledger\n')
        get_nym_response_json = await ledger.submit_request(pool_handle=pool_handle,
                                                           request_json=get_nym_request)
        get_nym_response = json.loads(get_nym_response_json)
        print_log('GET_NYM response: ')
        pprint.pprint(get_nym_response)

        # See whether we received the same info that we wrote the ledger in step 4.
        print_log('\n12. Comparing Trust Anchor verkey as written by Steward and as retrieved in GET_NYM '
                  'response submitted by Client\n')
        print_log('Written by Steward: ', trust_anchor_verkey)
        verkey_from_ledger = json.loads(get_nym_response['result']['data'])['verkey']
        print_log('Queried from ledger: ', verkey_from_ledger)
        print_log('Matching: ', verkey_from_ledger == trust_anchor_verkey)

        # Do some cleanup.
        print_log('\n13. Closing wallet and pool\n')
        await wallet.close_wallet(wallet_handle)
        await pool.close_pool_ledger(pool_handle)

        print_log('\n14. Deleting created wallet\n')
        await wallet.delete_wallet(wallet_name, None)

        print_log('\n15. Deleting pool ledger config\n')
        await pool.delete_pool_ledger_config(pool_name)

