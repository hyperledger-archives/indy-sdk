package org.hyperledger.indy.sdk.ledger;

import org.hyperledger.indy.sdk.IndyIntegrationTest;
import org.json.JSONObject;
import org.junit.Test;

public class AuthRuleRequestsTest extends IndyIntegrationTest {

	private String txnType = "NYM";
	private String authTypeCode = "1";
	private String field = "role";
	private String newValue = "101";
	private JSONObject constraint = new JSONObject()
			.put("sig_count", 1)
			.put("role", "0")
			.put("constraint_id", "ROLE")
			.put("need_to_be_owner", false);

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

	@Test
	public void testBuildAuthRuleRequestWorksForEditAction() throws Exception {
		String editAuthAction = "EDIT";
		String oldValue = "0";

		JSONObject expectedResult = new JSONObject()
				.put("identifier", DID)
				.put("operation",
						new JSONObject()
								.put("type", "120")
								.put("auth_type", authTypeCode)
								.put("auth_action", editAuthAction)
								.put("field", field)
								.put("old_value", oldValue)
								.put("new_value", newValue)
								.put("constraint", constraint)
				);

		String request = Ledger.buildAuthRuleRequest(DID, txnType, editAuthAction, field, oldValue, newValue, constraint.toString()).get();

		assert (new JSONObject(request).toMap().entrySet()
				.containsAll(
						expectedResult.toMap().entrySet()));
	}

	@Test
	public void testBuildGetAuthRuleRequestWorksForAddAction() throws Exception {
		String addAuthAction = "ADD";

		JSONObject expectedResult = new JSONObject()
				.put("identifier", DID)
				.put("operation",
						new JSONObject()
								.put("type", "121")
								.put("auth_type", authTypeCode)
								.put("auth_action", addAuthAction)
								.put("field", field)
								.put("new_value", newValue)
				);

		String request = Ledger.buildGetAuthRuleRequest(DID, txnType, addAuthAction, field, null, newValue).get();

		assert (new JSONObject(request).toMap().entrySet()
				.containsAll(
						expectedResult.toMap().entrySet()));
	}
}
