		print_log('\n9. Generating new verkey of trust anchor in wallet\n')
		new_verkey = await did.replace_keys_start(wallet_handle, trust_anchor_did, "{}")
		print_log('New Trust Anchor Verkey: ', new_verkey)
        
		print_log('\n10. Building NYM request to update new verkey to ledger\n')
		nym_request = await ledger.build_nym_request(trust_anchor_did, trust_anchor_did, new_verkey, None, 'TRUST_ANCHOR')
		print_log('NYM request:')
		pprint.pprint(json.loads(nym_request))
        
		print_log('\n11. Sending NYM request to the ledger\n')
		nym_response = await ledger.sign_and_submit_request(pool_handle, wallet_handle, trust_anchor_did, nym_request)
		print_log('NYM response:')
		pprint.pprint(json.loads(nym_response))
        
		print_log('\n12. Apply new verkey in wallet\n')
		await did.replace_keys_apply(wallet_handle, trust_anchor_did)
