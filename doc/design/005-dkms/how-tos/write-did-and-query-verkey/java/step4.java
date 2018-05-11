        // Here, we are building the transaction payload that we'll send to write the Trust Anchor identity to the ledger.
        // We submit this transaction under the authority of the steward DID that the ledger already recognizes.
        // This call will look up the private key of the steward DID in our wallet, and use it to sign the transaction.
        System.out.println("\n7. Build NYM request to add Trust Anchor to the ledger\n");
        String nymRequest = buildNymRequest(defautStewardDid, trustAnchorDID, trustAnchorVerkey, null, "TRUST_ANCHOR").get();
        System.out.println("NYM request JSON:\n" + nymRequest);

        // Now that we have the transaction ready, send it. The building and the sending are separate steps because some
        // clients may want to prepare transactions in one piece of code (e.g., that has access to privileged backend systems),
        // and communicate with the ledger in a different piece of code (e.g., that lives outside the safe internal
        // network).
        System.out.println("\n8. Sending the nym request to ledger\n");
        String nymResponseJson = signAndSubmitRequest(pool, walletHandle, defautStewardDid, nymRequest).get();
        System.out.println("NYM transaction response:\n" + nymResponseJson);

        // At this point, we have successfully written a new identity to the ledger. Our next step will be to query it.
