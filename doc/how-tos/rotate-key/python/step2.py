		# Tell SDK which pool you are going to use. You should have already started
		# this pool using docker compose or similar. Here, we are dumping the config
		# just for demonstration purposes.
		print_log('1. Creates a new local pool ledger configuration that is used '
				  'later when connecting to ledger.\n')
		pool_config = json.dumps({'genesis_txn': str(genesis_file_path)})
		await pool.create_pool_ledger_config(config_name=pool_name, config=pool_config)

		print_log('\n2. Open pool ledger and get handle from libindy\n')
		pool_handle = await pool.open_pool_ledger(config_name=pool_name, config=None)

		print_log('\n3. Creating new secure wallet with the given unique name\n')
		await wallet.create_wallet(wallet_config, wallet_credentials)

		print_log('\n4. Open wallet and get handle from libindy to use in methods that require wallet access\n')
		wallet_handle = await wallet.open_wallet(wallet_config, wallet_credentials)

		# First, put a steward DID and its keypair in the wallet. This doesn't write anything to the ledger,
		# but it gives us a key that we can use to sign a ledger transaction that we're going to submit later.
		print_log('\n5. Generating and storing steward DID and verkey\n')

		# The DID and public verkey for this steward key are already in the ledger; they were part of the genesis
		# transactions we told the SDK to start with in the previous step. But we have to also put the DID, verkey,
		# and private signing key into our wallet, so we can use the signing key to submit an acceptably signed
		# transaction to the ledger, creating our *next* DID (which is truly new). This is why we use a hard-coded seed
		# when creating this DID--it guarantees that the same DID and key material are created that the genesis txns
		# expect.
		steward_seed = "000000000000000000000000Steward1"

		did_json = json.dumps({'seed': steward_seed})

		steward_did, steward_verkey = await did.create_and_store_my_did(wallet_handle, did_json)
		print_log('Steward DID: ', steward_did)

		# Now, create a new DID and verkey for a trust anchor, and store it in our wallet as well. Don't use a seed;
		# this DID and its keys are secure and random. Again, we're not writing to the ledger yet.
		print_log('\n6. Generating and storing trust anchor DID and verkey\n')
		trust_anchor_did, trust_anchor_verkey = await did.create_and_store_my_did(wallet_handle, "{}")
		print_log('Trust Anchor DID: ', trust_anchor_did)
		print_log('Trust Anchor Verkey: ', trust_anchor_verkey)

		# Here, we are building the transaction payload that we'll send to write the Trust Anchor identity to the ledger.
		# We submit this transaction under the authority of the steward DID that the ledger already recognizes.
		# This call will look up the private key of the steward DID in our wallet, and use it to sign the transaction.
		print_log('\n7. Building NYM request to add Trust Anchor to the ledger\n')
		nym_transaction_request = await ledger.build_nym_request(submitter_did=steward_did,
									 target_did=trust_anchor_did,
									 ver_key=trust_anchor_verkey,
									 alias=None,
									 role='TRUST_ANCHOR')
		
		print_log('NYM request: ')
		pprint.pprint(json.loads(nym_transaction_request))

		# Now that we have the transaction ready, send it. The building and the sending are separate steps because some
		# clients may want to prepare transactions in one piece of code (e.g., that has access to privileged backend systems),
		# and communicate with the ledger in a different piece of code (e.g., that lives outside the safe internal
		# network).
		print_log('\n8. Sending NYM request to the ledger\n')
		nym_transaction_response = await ledger.sign_and_submit_request(pool_handle=pool_handle,
										wallet_handle=wallet_handle,
										submitter_did=steward_did,
										request_json=nym_transaction_request)
		
		print_log('NYM response: ')
		pprint.pprint(json.loads(nym_transaction_response))

		# At this point, we have successfully written a new identity to the ledger. Our next step will be to query it.
