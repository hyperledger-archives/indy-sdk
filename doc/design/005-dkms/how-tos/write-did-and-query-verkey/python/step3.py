        # First, put a steward DID and its keypair in the wallet. This doesn't write anything to the ledger,
        # but it gives us a key that we can use to sign a ledger transaction that we're going to submit later.
        print_log('\n5. Generate and store steward DID and verkey\n')
        # The DID and public verkey for this steward key are already in the ledger; they were part of the genesis
        # transactions we told the SDK to start with in the previous step. But we have to also put the DID, verkey,
        # and private signing key into our wallet, so we can use the signing key to submit an acceptably signed
        # transaction to the ledger, creating our *next* DID (which is truly new). This is why we use a hard-coded seed
        # when creating this DID--it guarantees that the same DID and key material are created that the genesis txns
        # expect.
        steward_seed = '000000000000000000000000Steward1'
        did_json = json.dumps({'seed': steward_seed})
        steward_did, steward_verkey = await signus.create_and_store_my_did(wallet_handle, did_json)
        print_log('Steward DID: ', steward_did)
        print_log('Steward Verkey: ', steward_verkey)

        # Now, create a new DID and verkey for a trust anchor, and store it in our wallet as well. Don't use a seed;
        # this DID and its keyas are secure and random. Again, we're not writing to the ledger yet.
        print_log('\n6. Generating and storing trust anchor DID and verkey\n')
        trust_anchor_did, trust_anchor_verkey = await signus.create_and_store_my_did(wallet_handle, "{}")
        print_log('Trust anchor DID: ', trust_anchor_did)
        print_log('Trust anchor Verkey: ', trust_anchor_verkey)
