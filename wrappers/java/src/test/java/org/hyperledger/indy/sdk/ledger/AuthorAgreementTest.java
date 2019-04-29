package org.hyperledger.indy.sdk.ledger;

import org.hyperledger.indy.sdk.IndyIntegrationTestWithPoolAndSingleWallet;
import org.hyperledger.indy.sdk.InvalidStructureException;
import org.hyperledger.indy.sdk.did.Did;
import org.hyperledger.indy.sdk.did.DidResults;
import org.hyperledger.indy.sdk.utils.PoolUtils;
import org.json.JSONObject;
import org.junit.Test;

import java.util.concurrent.ExecutionException;

import static org.hamcrest.CoreMatchers.isA;
import static org.junit.Assert.assertNotNull;
import static org.junit.Assert.assertTrue;

public class AuthorAgreementTest extends IndyIntegrationTestWithPoolAndSingleWallet {

	@Test
	public void testBuildAuthRuleRequestWorksForAddAction() throws Exception {
		String addAuthAction = "ADD";

		JSONObject expectedResult = new JSONObject()
				.put("identifier", DID)
				.put("operation",
						new JSONObject()
								.put("type", "120")
								.put("auth_type", authTypeCode)
								.put("auth_action", addAuthAction)
								.put("field", field)
								.put("new_value", newValue)
								.put("constraint", constraint)
				);

		String request = Ledger.buildAuthRuleRequest(DID, txnType, addAuthAction, field, null, newValue, constraint.toString()).get();

		assert (new JSONObject(request).toMap().entrySet()
				.containsAll(
						expectedResult.toMap().entrySet()));
	}
}