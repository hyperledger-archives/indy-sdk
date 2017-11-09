package org.hyperledger.indy.sdk.signus;

import org.hyperledger.indy.sdk.IndyIntegrationTestWithPoolAndSingleWallet;
import org.hyperledger.indy.sdk.InvalidStateException;
import org.hyperledger.indy.sdk.ledger.Ledger;
import org.junit.Test;

import java.util.concurrent.ExecutionException;

import static org.hamcrest.CoreMatchers.isA;
import static org.junit.Assert.assertEquals;


public class GetEndpointForDidTest extends IndyIntegrationTestWithPoolAndSingleWallet {

	@Test
	public void testGetEndpointForDidWorks() throws Exception {
		Signus.setEndpointForDid(wallet, DID, ENDPOINT, VERKEY).get();
		SignusResults.EndpointForDidResult receivedEndpoint = Signus.getEndpointForDid(wallet, pool, DID).get();
		assertEquals(ENDPOINT, receivedEndpoint.getAddress());
		assertEquals(VERKEY, receivedEndpoint.getTransportKey());
	}

	@Test
	public void testGetEndpointForDidWorksFromLedger() throws Exception {
		SignusResults.CreateAndStoreMyDidResult trusteeDidResult = Signus.createAndStoreMyDid(wallet, TRUSTEE_IDENTITY_JSON).get();
		String trusteeDid = trusteeDidResult.getDid();
		String trusteeVerkey = trusteeDidResult.getVerkey();

		String endpoint = String.format("{\"endpoint\":{\"ha\":\"%s\",\"verkey\":\"%s\"}}", ENDPOINT, trusteeVerkey);

		String attribRequest = Ledger.buildAttribRequest(trusteeDid, trusteeDid, null, endpoint, null).get();
		Ledger.signAndSubmitRequest(pool, wallet, trusteeDid, attribRequest).get();

		SignusResults.EndpointForDidResult receivedEndpoint = Signus.getEndpointForDid(wallet, pool, trusteeDid).get();
		assertEquals(ENDPOINT, receivedEndpoint.getAddress());
		assertEquals(trusteeVerkey, receivedEndpoint.getTransportKey());
	}

	@Test
	public void testGetEndpointForDidWorksForUnknownDid() throws Exception {
		thrown.expect(ExecutionException.class);
		thrown.expectCause(isA(InvalidStateException.class));

		Signus.getEndpointForDid(wallet, pool, DID).get();
	}
}