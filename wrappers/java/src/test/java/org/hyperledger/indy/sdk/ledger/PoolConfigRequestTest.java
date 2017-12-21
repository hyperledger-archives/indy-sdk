package org.hyperledger.indy.sdk.ledger;

import org.hyperledger.indy.sdk.IndyIntegrationTestWithPoolAndSingleWallet;
import org.hyperledger.indy.sdk.signus.Signus;
import org.hyperledger.indy.sdk.signus.SignusResults;
import org.junit.Test;

import static org.junit.Assert.*;

public class PoolConfigRequestTest extends IndyIntegrationTestWithPoolAndSingleWallet {

	@Test
	public void testBuildPoolConfigRequestWorks() throws Exception {
		String expectedResult = String.format("\"identifier\":\"%s\"," +
				"\"operation\":{" +
				"\"type\":\"111\"," +
				"\"writes\":false," +
				"\"force\":false" +
				"}", DID);

		String request = Ledger.buildPoolConfigRequest(DID, false, false).get();

		assertTrue(request.contains(expectedResult));
	}

	@Test
	public void testPoolConfigRequestWorks() throws Exception {
		SignusResults.CreateAndStoreMyDidResult didResult = Signus.createAndStoreMyDid(wallet, TRUSTEE_IDENTITY_JSON).get();
		String did = didResult.getDid();

		String request = Ledger.buildPoolConfigRequest(did, false, false).get();
		Ledger.signAndSubmitRequest(pool, wallet, did, request).get();

		request = Ledger.buildPoolConfigRequest(did, true, false).get();
		Ledger.signAndSubmitRequest(pool, wallet, did, request).get();
	}
}
