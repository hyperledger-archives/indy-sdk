package org.hyperledger.indy.sdk.ledger;

import org.hyperledger.indy.sdk.IndyIntegrationTest;
import org.hyperledger.indy.sdk.JsonObjectSimilar;
import org.json.JSONObject;
import org.junit.Test;

import static org.junit.Assert.assertEquals;
import static org.junit.Assert.assertTrue;


public class AppendAuthorAgreementMetaToRequestTest extends IndyIntegrationTest {


	private String text = "some agreement text";
	private String version = "1.0.0";
	private String acceptanceMechanismType = "acceptance type 1";
	private String hash = "050e52a57837fff904d3d059c8a123e3a04177042bf467db2b2c27abd8045d5e";
	private int timeOfAcceptance = 123456789;
	private String request = "{ \n" +
			"    \"reqId\": 1496822211362017764, \n" +
			"    \"identifier\": \"GJ1SzoWzavQYfNL9XkaJdrQejfztN4XqdsiV4ct3LXKL\", \n" +
			"    \"operation\": { \n" +
			"        \"type\": \"1\", \n" +
			"        \"dest\": \"VsKV7grR1BUE29mG2Fm2kX\", \n" +
			"        \"verkey\": \"GjZWsBLgZCR18aL468JAT7w9CZRiBnpxUPPgyQxh4voa\" \n" +
			"    } \n" +
			"}";

	private void checkRequestMeta(String request) {
		JSONObject expectedMeta = new JSONObject()
				.put("acceptanceMechanismType", acceptanceMechanismType)
				.put("hash", hash)
				.put("timeOfAcceptance", timeOfAcceptance);

		JSONObject actualRequest = new JSONObject(request).getJSONObject("txnAuthrAgrmtMeta");
		assertTrue(JsonObjectSimilar.similar(actualRequest, expectedMeta));
	}

	@Test
	public void testAppendAuthorAgreementMetaToRequestForTextVersion() throws Exception {
		String requestWithMeta = Ledger.appendTxnAuthorAgreementMetaToRequest(request, text, version, null, acceptanceMechanismType, timeOfAcceptance).get();
		checkRequestMeta(requestWithMeta);
	}

	@Test
	public void testAppendAuthorAgreementMetaToRequestForHash() throws Exception {
		String requestWithMeta = Ledger.appendTxnAuthorAgreementMetaToRequest(request, null, null, hash, acceptanceMechanismType, timeOfAcceptance).get();
		checkRequestMeta(requestWithMeta);
	}

	@Test
	public void testAppendAuthorAgreementMetaToRequestForTextVersionHash() throws Exception {
		String requestWithMeta = Ledger.appendTxnAuthorAgreementMetaToRequest(request, text, version, hash, acceptanceMechanismType, timeOfAcceptance).get();
		checkRequestMeta(requestWithMeta);
	}
}