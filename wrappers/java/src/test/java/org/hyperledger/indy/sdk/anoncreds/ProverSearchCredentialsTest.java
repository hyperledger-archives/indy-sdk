package org.hyperledger.indy.sdk.anoncreds;


import org.hyperledger.indy.sdk.wallet.WalletInvalidQueryException;
import org.json.JSONArray;
import org.json.JSONObject;
import org.junit.Test;

import java.util.concurrent.ExecutionException;

import static org.hamcrest.CoreMatchers.isA;
import static org.junit.Assert.assertEquals;

public class ProverSearchCredentialsTest extends AnoncredsIntegrationTest {

	@Test
	public void testProverSearchCredentialsWorksForEmptyFilter() throws Exception {

		JSONObject json = new JSONObject();
		String filter = json.toString();

		CredentialsSearch credentials = CredentialsSearch.open(wallet, filter).get();
		assertEquals(3, credentials.totalCount());

		JSONArray credentialsArray = new JSONArray(credentials.fetchNextCredentials(100).get());

		assertEquals(3, credentialsArray.length());
	}


	@Test
	public void testProverSearchCredentialsWorksForInvalidFilterJson() throws Exception {

		thrown.expect(ExecutionException.class);
		thrown.expectCause(isA(WalletInvalidQueryException.class));

		JSONObject json = new JSONObject();
		String filter = json.put("issuer_did", 1).toString();

		Anoncreds.proverGetCredentials(wallet, filter).get();
	}
}
