package org.hyperledger.indy.sdk.anoncreds;

import org.hyperledger.indy.sdk.InvalidStructureException;
import org.json.JSONObject;
import org.junit.*;

import java.util.concurrent.ExecutionException;

import static org.hamcrest.CoreMatchers.isA;
import static org.junit.Assert.assertNotNull;
import static org.junit.Assert.assertTrue;

public class IssuerCreateClaimTest extends AnoncredsIntegrationTest {

	@Test
	public void testIssuerCreateClaimWorks() throws Exception {

		initCommonWallet();

		String claimRequest = String.format(claimRequestTemplate, issuerDid, gvtSchemaKey);

		AnoncredsResults.IssuerCreateClaimResult createClaimResult = Anoncreds.issuerCreateClaim(wallet, claimRequest, gvtClaimValuesJson, - 1).get();
		assertNotNull(createClaimResult);
		String claimJson = createClaimResult.getClaimJson();

		JSONObject claimObj = new JSONObject(claimJson);

		JSONObject primaryClaim = claimObj.getJSONObject("signature").getJSONObject("p_claim");

		assertTrue(primaryClaim.getString("a").length() > 0);
		assertTrue(primaryClaim.getString("m_2").length() > 0);
		assertTrue(primaryClaim.getString("e").length() > 0);
		assertTrue(primaryClaim.getString("v").length() > 0);
	}

	@Test
	public void testIssuerCreateClaimWorksForClaimDoesNotCorrespondToClaimRequest() throws Exception {

		initCommonWallet();

		thrown.expect(ExecutionException.class);
		thrown.expectCause(isA(InvalidStructureException.class));

		String claimRequest = String.format(claimRequestTemplate, issuerDid, 1);

		Anoncreds.issuerCreateClaim(wallet, claimRequest, xyzClaimValuesJson, - 1).get();
	}

	@Test
	public void testIssuerCreateClaimWorksForInvalidClaimValues() throws Exception {

		initCommonWallet();

		thrown.expect(ExecutionException.class);
		thrown.expectCause(isA(InvalidStructureException.class));

		String claimRequest = String.format(claimRequestTemplate, issuerDid, 1);

		String claim = "{" +
				"        \"sex\":\"male\",\n" +
				"        \"name\":\"Alex\",\n" +
				"        \"height\":\"175\",\n" +
				"        \"age\":\"28\"" +
				"       }";

		Anoncreds.issuerCreateClaim(wallet, claimRequest, claim, - 1).get();
	}
}
