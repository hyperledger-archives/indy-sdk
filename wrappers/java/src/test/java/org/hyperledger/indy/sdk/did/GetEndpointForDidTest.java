package org.hyperledger.indy.sdk.did;

import org.hyperledger.indy.sdk.IndyIntegrationTestWithPoolAndSingleWallet;
import org.hyperledger.indy.sdk.InvalidStateException;
import org.hyperledger.indy.sdk.ledger.Ledger;
import org.hyperledger.indy.sdk.utils.PoolUtils;
import org.junit.Test;

import java.util.concurrent.ExecutionException;

import static org.hamcrest.CoreMatchers.isA;
import static org.junit.Assert.assertEquals;
import static org.junit.Assert.assertTrue;


public class GetEndpointForDidTest extends IndyIntegrationTestWithPoolAndSingleWallet {

	@Test
	public void testGetEndpointForDidWorks() throws Exception {
		Did.setEndpointForDid(wallet, DID, ENDPOINT, VERKEY).get();
		DidResults.EndpointForDidResult receivedEndpoint = Did.getEndpointForDid(wallet, pool, DID).get();
		assertEquals(ENDPOINT, receivedEndpoint.getAddress());
		assertEquals(VERKEY, receivedEndpoint.getTransportKey());
	}

	@Test(timeout = PoolUtils.TEST_TIMEOUT_FOR_REQUEST_ENSURE)
	public void testGetEndpointForDidWorksFromLedger() throws Exception {
		DidResults.CreateAndStoreMyDidResult trusteeDidResult = Did.createAndStoreMyDid(wallet, TRUSTEE_IDENTITY_JSON).get();
		String trusteeDid = trusteeDidResult.getDid();
		String trusteeVerkey = trusteeDidResult.getVerkey();

		String endpoint = String.format("{\"endpoint\":{\"ha\":\"%s\",\"verkey\":\"%s\"}}", ENDPOINT, trusteeVerkey);

		String attribRequest = Ledger.buildAttribRequest(trusteeDid, trusteeDid, null, endpoint, null).get();
		Ledger.signAndSubmitRequest(pool, wallet, trusteeDid, attribRequest).get();

		assertTrue(PoolUtils.retryCheck(
						() -> Did.getEndpointForDid(wallet, pool, trusteeDid).get().toString(),
						didEndpoint -> didEndpoint.equals(new DidResults.EndpointForDidResult(ENDPOINT, trusteeVerkey).toString())));
	}

	@Test
	public void testGetEndpointForDidWorksForUnknownDid() throws Exception {
		thrown.expect(ExecutionException.class);
		thrown.expectCause(isA(InvalidStateException.class));

		Did.getEndpointForDid(wallet, pool, DID).get();
	}
}