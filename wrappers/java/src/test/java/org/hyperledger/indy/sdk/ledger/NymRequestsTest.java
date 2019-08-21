package org.hyperledger.indy.sdk.ledger;

import org.hyperledger.indy.sdk.IndyIntegrationTestWithPoolAndSingleWallet;
import org.hyperledger.indy.sdk.InvalidStructureException;
import org.hyperledger.indy.sdk.did.Did;
import org.hyperledger.indy.sdk.did.DidJSONParameters;
import org.hyperledger.indy.sdk.did.DidResults;
import org.hyperledger.indy.sdk.utils.PoolUtils;
import org.json.JSONObject;
import org.junit.*;

import java.util.concurrent.ExecutionException;

import static org.hamcrest.CoreMatchers.isA;
import static org.junit.Assert.assertEquals;
import static org.junit.Assert.assertNotNull;
import static org.junit.Assert.assertTrue;

public class NymRequestsTest extends IndyIntegrationTestWithPoolAndSingleWallet {

	private String dest = "FYmoFw55GeQH7SRFa37dkx1d2dZ3zUF8ckg7wmL7ofN4";
	private String role = "STEWARD";
	private String alias = "some_alias";

	@Test
	public void testBuildNymRequestWorksForOnlyRequiredFields() throws Exception {
		String expectedResult = String.format("\"identifier\":\"%s\",\"operation\":{\"dest\":\"%s\",\"type\":\"1\"}", DID, dest);

		String nymRequest = Ledger.buildNymRequest(DID, dest, null, null, null).get();
		assertTrue(nymRequest.contains(expectedResult));
	}

	@Test
	public void testBuildNymRequestWorksForEmptyRole() throws Exception {
		String expectedResult = String.format("\"identifier\":\"%s\",\"operation\":{\"dest\":\"%s\",\"role\":null,\"type\":\"1\"}", DID, dest);

		String nymRequest = Ledger.buildNymRequest(DID, dest, null, null, "").get();
		assertTrue(nymRequest.contains(expectedResult));
	}

	@Test
	public void testBuildNymRequestWorksForOnlyOptionalFields() throws Exception {
		String expectedResult = String.format("\"identifier\":\"%s\"," +
				"\"operation\":{" +
				"\"alias\":\"%s\"," +
				"\"dest\":\"%s\"," +
				"\"role\":\"2\"," +
				"\"type\":\"1\"," +
				"\"verkey\":\"%s\"" +
				"}", DID, alias, dest, VERKEY_TRUSTEE);

		String nymRequest = Ledger.buildNymRequest(DID, dest, VERKEY_TRUSTEE, alias, role).get();
		assertTrue(nymRequest.contains(expectedResult));
	}

	@Test
	public void testBuildGetNymRequestWorks() throws Exception {
		String expectedResult = String.format("\"identifier\":\"%s\",\"operation\":{\"type\":\"105\",\"dest\":\"%s\"}", DID, dest);

		String nymRequest = Ledger.buildGetNymRequest(DID, dest).get();
		assertTrue(nymRequest.contains(expectedResult));
	}

	@Test
	public void testBuildGetNymRequestWorksForDefaultSubmitter() throws Exception {
		Ledger.buildGetNymRequest(null, dest).get();
	}

	@Test(timeout = PoolUtils.TEST_TIMEOUT_FOR_REQUEST_ENSURE)
	public void testNymRequestsWorks() throws Exception {
		DidResults.CreateAndStoreMyDidResult trusteeDidResult = Did.createAndStoreMyDid(wallet, TRUSTEE_IDENTITY_JSON).get();
		String trusteeDid = trusteeDidResult.getDid();

		DidResults.CreateAndStoreMyDidResult myDidResult = Did.createAndStoreMyDid(wallet, "{}").get();
		String myDid = myDidResult.getDid();
		String myVerkey = myDidResult.getVerkey();

		String nymRequest = Ledger.buildNymRequest(trusteeDid, myDid, myVerkey, null, null).get();
		Ledger.signAndSubmitRequest(pool, wallet, trusteeDid, nymRequest).get();

		String getNymRequest = Ledger.buildGetNymRequest(myDid, myDid).get();
		String getNymResponse = PoolUtils.ensurePreviousRequestApplied(pool, getNymRequest,
				response -> compareResponseType(response, "REPLY"));
		assertNotNull(getNymResponse);
	}
}
