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

		String claimRequest = String.format(claimRequestTemplate, issuerDid, 1);

		String claim = "{\"sex\":[\"male\",\"5944657099558967239210949258394887428692050081607692519917050011144233115103\"],\n" +
				"               \"name\":[\"Alex\",\"1139481716457488690172217916278103335\"],\n" +
				"               \"height\":[\"175\",\"175\"],\n" +
				"               \"age\":[\"28\",\"28\"]\n" +
				"        }";

		AnoncredsResults.IssuerCreateClaimResult createClaimResult = Anoncreds.issuerCreateClaim(wallet, claimRequest, claim, - 1).get();
		assertNotNull(createClaimResult);
		String claimJson = createClaimResult.getClaimJson();

		JSONObject claimObj = new JSONObject(claimJson);

		JSONObject primaryClaim = claimObj.getJSONObject("signature").getJSONObject("primary_claim");

		assertTrue(primaryClaim.getString("a").length() > 0);
		assertTrue(primaryClaim.getString("m2").length() > 0);
		assertTrue(primaryClaim.getString("e").length() > 0);
		assertTrue(primaryClaim.getString("v").length() > 0);
	}

	@Test
	public void testIssuerCreateClaimWorksForClaimDoesNotCorrespondToClaimRequest() throws Exception {

		initCommonWallet();

		thrown.expect(ExecutionException.class);
		thrown.expectCause(isA(InvalidStructureException.class));

		String claimRequest = String.format(claimRequestTemplate, issuerDid, 1);

		String claim = "{\"status\":[\"partial\",\"51792877103171595686471452153480627530895\"],\n" +
				"        \"period\":[\"8\",\"8\"]\n" +
				"       }";

		Anoncreds.issuerCreateClaim(wallet, claimRequest, claim, - 1).get();
	}

	@Test
	public void testIssuerCreateClaimWorksForInvalidClaim() throws Exception {

		initCommonWallet();

		thrown.expect(ExecutionException.class);
		thrown.expectCause(isA(InvalidStructureException.class));

		String claimRequest = String.format(claimRequestTemplate, issuerDid, 1);

		String claim = "{\"sex\":\"male\",\n" +
				"        \"name\":\"Alex\",\n" +
				"        \"height\":\"175\",\n" +
				"        \"age\":\"28\"" +
				"       }";

		Anoncreds.issuerCreateClaim(wallet, claimRequest, claim, - 1).get();
	}
}
