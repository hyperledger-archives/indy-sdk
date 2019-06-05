package org.hyperledger.indy.sdk.ledger;

import org.hyperledger.indy.sdk.IndyIntegrationTest;
import org.json.JSONObject;
import org.junit.Test;

import java.util.ArrayList;
import java.util.Vector;

public class AuthRuleRequestsTest extends IndyIntegrationTest {

	private String txnType = "NYM";
	private String authTypeCode = "1";
	private String addAuthAction = "ADD";
	private String editAuthAction = "EDIT";
	private String field = "role";
	private String oldValue = "0";
	private String newValue = "101";
	private JSONObject constraint = new JSONObject()
			.put("sig_count", 1)
			.put("role", "0")
			.put("constraint_id", "ROLE")
			.put("need_to_be_owner", false);

	@Test
	public void testBuildAuthRuleRequestWorksForAddAction() throws Exception {
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

	@Test
	public void testBuildAuthRulesRequestWorks() throws Exception {
		ArrayList<JSONObject> data = new ArrayList<>();

		data.add(new JSONObject()
				.put("auth_type", txnType)
				.put("auth_action", addAuthAction)
				.put("field", field)
				.put("new_value", newValue)
				.put("constraint", constraint)
		);

		data.add(new JSONObject()
				.put("auth_type", txnType)
				.put("auth_action", editAuthAction)
				.put("field", field)
				.put("old_value", oldValue)
				.put("new_value", newValue)
				.put("constraint", constraint)
		);

		JSONObject expectedResult = new JSONObject()
				.put("identifier", DID)
				.put("operation",
						new JSONObject()
								.put("type", "122")
								.put("rules", data)
				);

		String request = Ledger.buildAuthRulesRequest(DID, data.toString()).get();

		assert (new JSONObject(request).toMap().entrySet()
				.containsAll(
						expectedResult.toMap().entrySet()));
	}
}
