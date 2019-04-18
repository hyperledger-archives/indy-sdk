package org.hyperledger.indy.sdk.anoncreds;


import org.hyperledger.indy.sdk.wallet.WalletItemNotFoundException;
import org.json.JSONObject;
import org.junit.Test;

import java.util.concurrent.ExecutionException;

import static org.hamcrest.CoreMatchers.isA;
import static org.junit.Assert.assertTrue;

import org.hyperledger.indy.sdk.JsonObjectSimilar;

public class ProverDeleteCredentialTest extends AnoncredsIntegrationTest {

	@Test
	public void testProverDeleteCredentialWorks() throws Exception {
		Anoncreds.proverDeleteCredential(wallet, credentialIdX).get();

		thrown.expect(ExecutionException.class);
		thrown.expectCause(isA(WalletItemNotFoundException.class));

		Anoncreds.proverGetCredential(wallet, credentialIdX).get();  // make sure it's gone

		thrown.expect(ExecutionException.class);
		thrown.expectCause(isA(WalletItemNotFoundException.class));

		Anoncreds.proverDeleteCredential(wallet, credentialIdX).get();  // exercise double deletion
	}
}
