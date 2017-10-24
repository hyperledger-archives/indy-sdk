package org.hyperledger.indy.sdk.ledger;

import org.hyperledger.indy.sdk.IndyIntegrationTestWithPoolAndSingleWallet;
import org.hyperledger.indy.sdk.InvalidStructureException;
import org.hyperledger.indy.sdk.signus.Signus;
import org.hyperledger.indy.sdk.signus.SignusJSONParameters;
import org.hyperledger.indy.sdk.signus.SignusResults;
import org.junit.*;

import java.util.concurrent.ExecutionException;

import static org.hamcrest.CoreMatchers.isA;
import static org.junit.Assert.assertTrue;

public class NodeRequestsTest extends IndyIntegrationTestWithPoolAndSingleWallet {

	private String dest = "A5iWQVT3k8Zo9nXj4otmeqaUziPQPCiDqcydXkAJBk1Y";
	private String data = "{\"node_ip\":\"10.0.0.100\"," +
			"\"node_port\":910," +
			"\"client_ip\":\"10.0.0.100\"," +
			"\"client_port\":911," +
			"\"alias\":\"some\"," +
			"\"services\":[\"VALIDATOR\"]," +
			"\"blskey\":\"CnEDk9HrMnmiHXEV1WFgbVCRteYnPqsJwrTdcZaNhFVW\"}";

	private SignusJSONParameters.CreateAndStoreMyDidJSONParameter stewardDidJson =
			new SignusJSONParameters.CreateAndStoreMyDidJSONParameter(null, "000000000000000000000000Steward1", null, null);

	@Test
	public void testBuildNodeRequestWorks() throws Exception {
		String expectedResult = String.format("\"identifier\":\"%s\"," +
				"\"operation\":{" +
				"\"type\":\"0\"," +
				"\"dest\":\"%s\"," +
				"\"data\":%s" +
				"}", DID1, dest, data);

		String nodeRequest = Ledger.buildNodeRequest(DID1, dest, data).get();

		assertTrue(nodeRequest.replace("\\", "").contains(expectedResult));
	}

	@Test
	public void testSendNodeRequestWorksWithoutSignature() throws Exception {
		thrown.expect(ExecutionException.class);
		thrown.expectCause(isA(InvalidLedgerTransactionException.class));

		SignusResults.CreateAndStoreMyDidResult didResult = Signus.createAndStoreMyDid(wallet, stewardDidJson.toJson()).get();
		String did = didResult.getDid();

		String nodeRequest = Ledger.buildNodeRequest(did, did, data).get();
		Ledger.submitRequest(pool, nodeRequest).get();
	}

	@Test
	public void testBuildNodeRequestWorksForWrongServiceType() throws Exception {
		thrown.expect(ExecutionException.class);
		thrown.expectCause(isA(InvalidStructureException.class));

		String data = "{\"node_ip\":\"10.0.0.100\"," +
				"\"node_port\":910," +
				"\"client_ip\":\"10.0.0.100\"," +
				"\"client_port\":911," +
				"\"alias\":\"some\"," +
				"\"services\":[\"SERVICE\"]" +
				"\"blskey\":\"CnEDk9HrMnmiHXEV1WFgbVCRteYnPqsJwrTdcZaNhFVW\"}";

		Ledger.buildNodeRequest(DID1, dest, data).get();
	}

	@Test
	public void testBuildNodeRequestWorksForMissedField() throws Exception {
		thrown.expect(ExecutionException.class);
		thrown.expectCause(isA(InvalidStructureException.class));

		String data = "{\"node_ip\":\"10.0.0.100\"," +
				"\"node_port\":910," +
				"\"client_ip\":\"10.0.0.100\"," +
				"\"client_port\":910," +
				"\"services\":[\"VALIDATOR\"]}";

		Ledger.buildNodeRequest(DID1, dest, data).get();
	}

	@Test
	public void testSendNodeRequestWorksForWrongRole() throws Exception {
		thrown.expect(ExecutionException.class);
		thrown.expectCause(isA(InvalidLedgerTransactionException.class));

		SignusResults.CreateAndStoreMyDidResult didResult = Signus.createAndStoreMyDid(wallet, TRUSTEE_IDENTITY_JSON).get();
		String did = didResult.getDid();

		String nodeRequest = Ledger.buildNodeRequest(did, did, data).get();
		Ledger.signAndSubmitRequest(pool, wallet, did, nodeRequest).get();
	}

	@Test
	@Ignore
	public void testSendNodeRequestWorksForNewSteward() throws Exception {
		SignusResults.CreateAndStoreMyDidResult didResult = Signus.createAndStoreMyDid(wallet, TRUSTEE_IDENTITY_JSON).get();
		String trusteeDid = didResult.getDid();

		SignusResults.CreateAndStoreMyDidResult myDidResult = Signus.createAndStoreMyDid(wallet, "{}").get();
		String myDid = myDidResult.getDid();
		String myVerkey = myDidResult.getVerkey();

		String nymRequest = Ledger.buildNymRequest(trusteeDid, myDid, myVerkey, null, "STEWARD").get();
		Ledger.signAndSubmitRequest(pool, wallet, trusteeDid, nymRequest).get();

		String nodeRequest = Ledger.buildNodeRequest(myDid, dest, data).get();
		Ledger.signAndSubmitRequest(pool, wallet, myDid, nodeRequest).get();
	}
}
