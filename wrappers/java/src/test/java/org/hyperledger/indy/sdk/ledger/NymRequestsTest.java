package org.hyperledger.indy.sdk.ledger;

import org.hyperledger.indy.sdk.IndyIntegrationTestWithPoolAndSingleWallet;
import org.hyperledger.indy.sdk.InvalidStructureException;
import org.hyperledger.indy.sdk.signus.Signus;
import org.hyperledger.indy.sdk.signus.SignusJSONParameters;
import org.hyperledger.indy.sdk.signus.SignusResults;
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
		String verkey = "Anfh2rjAcxkE249DcdsaQl";

		String expectedResult = String.format("\"identifier\":\"%s\"," +
				"\"operation\":{" +
				"\"alias\":\"%s\"," +
				"\"dest\":\"%s\"," +
				"\"role\":\"2\"," +
				"\"type\":\"1\"," +
				"\"verkey\":\"%s\"" +
				"}", DID, alias, dest, verkey);

		String nymRequest = Ledger.buildNymRequest(DID, dest, verkey, alias, role).get();
		assertTrue(nymRequest.contains(expectedResult));
	}

	@Test
	public void testBuildGetNymRequestWorks() throws Exception {
		String expectedResult = String.format("\"identifier\":\"%s\",\"operation\":{\"type\":\"105\",\"dest\":\"%s\"}", DID, dest);

		String nymRequest = Ledger.buildGetNymRequest(DID, dest).get();
		assertTrue(nymRequest.contains(expectedResult));
	}

	@Test
	public void testNymRequestWorksWithoutSignature() throws Exception {
		thrown.expect(ExecutionException.class);
		thrown.expectCause(isA(InvalidLedgerTransactionException.class));

		SignusResults.CreateAndStoreMyDidResult result = Signus.createAndStoreMyDid(wallet, "{}").get();
		String did = result.getDid();

		String nymRequest = Ledger.buildNymRequest(did, did, null, null, null).get();
		Ledger.submitRequest(pool, nymRequest).get();
	}

	@Test
	public void testSendNymRequestsWorksForOnlyRequiredFields() throws Exception {
		SignusResults.CreateAndStoreMyDidResult trusteeDidResult = Signus.createAndStoreMyDid(wallet, TRUSTEE_IDENTITY_JSON).get();
		String trusteeDid = trusteeDidResult.getDid();

		SignusResults.CreateAndStoreMyDidResult myDidResult = Signus.createAndStoreMyDid(wallet, "{}").get();
		String myDid = myDidResult.getDid();

		String nymRequest = Ledger.buildNymRequest(trusteeDid, myDid, null, null, null).get();
		String nymResponse = Ledger.signAndSubmitRequest(pool, wallet, trusteeDid, nymRequest).get();
		assertNotNull(nymResponse);
	}

	@Test
	public void testSendNymRequestsWorksForOptionalFields() throws Exception {
		SignusResults.CreateAndStoreMyDidResult trusteeDidResult = Signus.createAndStoreMyDid(wallet, TRUSTEE_IDENTITY_JSON).get();
		String trusteeDid = trusteeDidResult.getDid();

		SignusResults.CreateAndStoreMyDidResult myDidResult = Signus.createAndStoreMyDid(wallet, "{}").get();
		String myDid = myDidResult.getDid();
		String myVerkey = myDidResult.getVerkey();

		String nymRequest = Ledger.buildNymRequest(trusteeDid, myDid, myVerkey, alias, role).get();
		String nymResponse = Ledger.signAndSubmitRequest(pool, wallet, trusteeDid, nymRequest).get();
		assertNotNull(nymResponse);
	}

	@Test
	public void testGetNymRequestWorks() throws Exception {
		SignusResults.CreateAndStoreMyDidResult result = Signus.createAndStoreMyDid(wallet, TRUSTEE_IDENTITY_JSON).get();
		String did = result.getDid();

		String getNymRequest = Ledger.buildGetNymRequest(did, did).get();
		String getNymResponseJson = Ledger.submitRequest(pool, getNymRequest).get();

		JSONObject getNymResponse = new JSONObject(getNymResponseJson);
		assertEquals(did, getNymResponse.getJSONObject("result").getString("dest"));
	}

	@Test
	public void testSendNymRequestsWorksForWrongSignerRole() throws Exception {
		thrown.expect(ExecutionException.class);
		thrown.expectCause(isA(InvalidLedgerTransactionException.class));

		SignusResults.CreateAndStoreMyDidResult trusteeDidResult = Signus.createAndStoreMyDid(wallet, TRUSTEE_IDENTITY_JSON).get();
		String trusteeDid = trusteeDidResult.getDid();

		SignusResults.CreateAndStoreMyDidResult myDidResult = Signus.createAndStoreMyDid(wallet, "{}").get();
		String myDid = myDidResult.getDid();

		String nymRequest = Ledger.buildNymRequest(trusteeDid, myDid, null, null, null).get();
		Ledger.signAndSubmitRequest(pool, wallet, trusteeDid, nymRequest).get();

		SignusResults.CreateAndStoreMyDidResult myDidResult2 = Signus.createAndStoreMyDid(wallet, "{}").get();
		String myDid2 = myDidResult2.getDid();

		String nymRequest2 = Ledger.buildNymRequest(myDid, myDid2, null, null, null).get();
		Ledger.signAndSubmitRequest(pool, wallet, myDid, nymRequest2).get();
	}

	@Test
	public void testSendNymRequestsWorksForUnknownSigner() throws Exception {
		thrown.expect(ExecutionException.class);
		thrown.expectCause(isA(InvalidLedgerTransactionException.class));

		String identityJson =
				new SignusJSONParameters.CreateAndStoreMyDidJSONParameter(null, "000000000000000000000000Trustee9", null, null).toJson();

		SignusResults.CreateAndStoreMyDidResult trusteeDidResult = Signus.createAndStoreMyDid(wallet, identityJson).get();
		String trusteeDid = trusteeDidResult.getDid();

		SignusResults.CreateAndStoreMyDidResult myDidResult = Signus.createAndStoreMyDid(wallet, "{}").get();
		String myDid = myDidResult.getDid();

		String nymRequest = Ledger.buildNymRequest(trusteeDid, myDid, null, null, null).get();
		Ledger.signAndSubmitRequest(pool, wallet, trusteeDid, nymRequest).get();
	}

	@Test
	public void testNymRequestsWorks() throws Exception {
		SignusResults.CreateAndStoreMyDidResult trusteeDidResult = Signus.createAndStoreMyDid(wallet, TRUSTEE_IDENTITY_JSON).get();
		String trusteeDid = trusteeDidResult.getDid();

		SignusResults.CreateAndStoreMyDidResult myDidResult = Signus.createAndStoreMyDid(wallet, "{}").get();
		String myDid = myDidResult.getDid();
		String myVerkey = myDidResult.getVerkey();

		String nymRequest = Ledger.buildNymRequest(trusteeDid, myDid, myVerkey, null, null).get();
		Ledger.signAndSubmitRequest(pool, wallet, trusteeDid, nymRequest).get();

		String getNymRequest = Ledger.buildGetNymRequest(myDid, myDid).get();
		String getNymResponseJson = Ledger.submitRequest(pool, getNymRequest).get();

		JSONObject getNymResponse = new JSONObject(getNymResponseJson);

		assertEquals("REPLY", getNymResponse.getString("op"));
		assertEquals("105", getNymResponse.getJSONObject("result").getString("type"));
		assertEquals(myDid, getNymResponse.getJSONObject("result").getString("dest"));
	}

	@Test
	public void testSendNymRequestsWorksForWrongRole() throws Exception {
		thrown.expect(ExecutionException.class);
		thrown.expectCause(isA(InvalidStructureException.class));

		Ledger.buildNymRequest(DID, dest, null, null, "WRONG_ROLE").get();
	}
}
