package org.hyperledger.indy.sdk.anoncreds;

import org.hyperledger.indy.sdk.ErrorCode;
import org.hyperledger.indy.sdk.ErrorCodeMatcher;
import org.json.JSONObject;
import org.junit.*;

import java.util.concurrent.ExecutionException;

import static org.junit.Assert.assertNotNull;

public class IssuerCreateClaimTest extends AnoncredsIntegrationTest {

	@Test
	public void testProverCreateAndStoreClaimReqWorks() throws Exception {

		initCommonWallet();

		String claimRequest = String.format(claimRequestTemplate, issuerDid, 1);

		String claim = "{\"sex\":[\"male\",\"5944657099558967239210949258394887428692050081607692519917050011144233115103\"],\n" +
				"               \"name\":[\"Alex\",\"1139481716457488690172217916278103335\"],\n" +
				"               \"height\":[\"175\",\"175\"],\n" +
				"               \"age\":[\"28\",\"28\"]\n" +
				"        }";

		AnoncredsResults.IssuerCreateClaimResult createClaimResult = Anoncreds.issuerCreateClaim(wallet, claimRequest, claim, - 1, - 1).get();
		assertNotNull(createClaimResult);
		String claimJson = createClaimResult.getClaimJson();

		JSONObject claimObj = new JSONObject(claimJson);

		JSONObject primaryClaim = claimObj.getJSONObject("signature").getJSONObject("primary_claim");

		Assert.assertTrue(primaryClaim.getString("a").length() > 0);
		Assert.assertTrue(primaryClaim.getString("m2").length() > 0);
		Assert.assertTrue(primaryClaim.getString("e").length() > 0);
		Assert.assertTrue(primaryClaim.getString("v").length() > 0);
	}

	@Test
	public void testProverCreateAndStoreClaimReqWorksForClaimDoesNotCorrespondToClaimRequest() throws Exception {

		initCommonWallet();

		thrown.expect(ExecutionException.class);
		thrown.expectCause(new ErrorCodeMatcher(ErrorCode.CommonInvalidStructure));

		String claimRequest = String.format(claimRequestTemplate, issuerDid, 1);

		String claim = "{\"status\":[\"partial\",\"51792877103171595686471452153480627530895\"],\n" +
				"        \"period\":[\"8\",\"8\"]\n" +
				"       }";

		Anoncreds.issuerCreateClaim(wallet, claimRequest, claim, - 1, - 1).get();
	}

	@Test
	public void testProverCreateAndStoreClaimReqWorksForInvalidClaim() throws Exception {

		initCommonWallet();

		thrown.expect(ExecutionException.class);
		thrown.expectCause(new ErrorCodeMatcher(ErrorCode.CommonInvalidStructure));

		String claimRequest = String.format(claimRequestTemplate, issuerDid, 1);

		String claim = "{\"sex\":\"male\",\n" +
				"        \"name\":\"Alex\",\n" +
				"        \"height\":\"175\",\n" +
				"        \"age\":\"28\"" +
				"       }";

		Anoncreds.issuerCreateClaim(wallet, claimRequest, claim, - 1, - 1).get();
	}
}
