        print_log('\n11.Verifier is verifying proof from Prover\n')
        assert await
        anoncreds.verifier_verify_proof(proof_req_json, proof_json, schemas_json, claim_defs_json, revoc_regs_json)

        # 12
        print_log('\n12. Closing both wallet_handles\n')
        await
        wallet.close_wallet(issuer_wallet_handle)
        await
        wallet.close_wallet(prover_wallet_handle)

        # 13
        print_log('\n13. Deleting created wallet_handles\n')
        await
        wallet.delete_wallet(prover_wallet_name, None)
        await
        wallet.delete_wallet(issuer_wallet_name, None)
