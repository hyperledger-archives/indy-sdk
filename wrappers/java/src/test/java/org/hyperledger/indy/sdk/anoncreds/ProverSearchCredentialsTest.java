package org.hyperledger.indy.sdk.anoncreds;


import org.json.JSONArray;
import org.json.JSONObject;
import org.junit.Test;

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
}
