		print_log('\n13. Reading new verkey from wallet\n')
		verkey_in_wallet = await did.key_for_local_did(wallet_handle, trust_anchor_did)
		print_log('Trust Anchor Verkey in wallet: ', verkey_in_wallet)

		print_log('\n14. Building GET_NYM request to get Trust Anchor verkey\n')
		get_nym_request = await ledger.build_get_nym_request(trust_anchor_did, trust_anchor_did)
		print_log('Get NYM request:')
		pprint.pprint(json.loads(get_nym_request))

		print_log('\n15. Sending GET_NYM request to ledger\n')
		get_nym_response_json = await ledger.submit_request(pool_handle, get_nym_request)
		get_nym_response = json.loads(get_nym_response_json)
		print_log('GET NYM response:')
		pprint.pprint(get_nym_response)

		print_log('\n16. Comparing Trust Anchor verkeys: written by Steward (original), '
		 'current in wallet and current from ledger\n')
		print_log('Written by Steward: ', trust_anchor_verkey)
		print_log('Current in wallet: ', verkey_in_wallet)
		verkey_from_ledger = json.loads(get_nym_response['result']['data'])['verkey']
		print_log('Current from ledger: ', verkey_from_ledger)
		print_log('Matching: ', verkey_from_ledger == verkey_in_wallet != trust_anchor_verkey)

		# Do some cleanup.
		print_log('\n17. Closing wallet and pool\n')
		await wallet.close_wallet(wallet_handle)
		await pool.close_pool_ledger(pool_handle)

		print_log('\n18. Deleting created wallet\n')
		await wallet.delete_wallet(wallet_config, wallet_credentials)

		print_log('\n19. Deleting pool ledger config')
		await pool.delete_pool_ledger_config(pool_name)
