package org.hyperledger.indy.sdk.ledger;

import org.hyperledger.indy.sdk.IndyIntegrationTest;
import org.json.JSONObject;
import org.junit.Test;

import java.util.HashMap;


public class AcceptanceMechanismRequestTest extends IndyIntegrationTest {

	@Test
	public void testBuildAcceptanceMechanismRequest() throws Exception {
		JSONObject aml = new JSONObject()
				.put("acceptance mechanism label 1", "some acceptance mechanism description 1");

		String amlContext = "some context";

		JSONObject expectedResult = new JSONObject()
				.put("identifier", DID)
				.put("operation",
						new JSONObject()
								.put("type", "5")
								.put("aml", aml)
								.put("amlContext", amlContext)
				);

		String request = Ledger.buildAcceptanceMechanismRequest(DID, aml.toString(), amlContext).get();

		assert (new JSONObject(request).toMap().entrySet()
				.containsAll(
						expectedResult.toMap().entrySet()));
	}

	@Test
	public void testBuildGetAcceptanceMechanismRequest() throws Exception {
		int timestamp = 123456789;

		JSONObject expectedResult = new JSONObject()
				.put("operation",
						new JSONObject()
								.put("type", "7")
								.put("timestamp", timestamp)
				);

		String request = Ledger.buildGetAcceptanceMechanismRequest(null, timestamp).get();

		assert (new JSONObject(request).toMap().entrySet()
				.containsAll(
						expectedResult.toMap().entrySet()));
	}
}