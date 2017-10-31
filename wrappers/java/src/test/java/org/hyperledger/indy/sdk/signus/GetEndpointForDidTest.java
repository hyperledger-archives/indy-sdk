package org.hyperledger.indy.sdk.signus;

import org.hyperledger.indy.sdk.IndyIntegrationTestWithSingleWallet;
import org.hyperledger.indy.sdk.wallet.WalletValueNotFoundException;
import org.junit.Test;

import java.util.concurrent.ExecutionException;

import static org.hamcrest.CoreMatchers.isA;
import static org.junit.Assert.assertEquals;


public class GetEndpointForDidTest extends IndyIntegrationTestWithSingleWallet {

	@Test
	public void testGetEndpointForDidWorks() throws Exception {
		Signus.setEndpointForDid(wallet, DID, ENDPOINT, VERKEY).get();
		SignusResults.EndpointForDidResult receivedEndpoint = Signus.getEndpointForDid(wallet, DID).get();
		assertEquals(ENDPOINT, receivedEndpoint.getAddress());
		assertEquals(VERKEY, receivedEndpoint.getTransportKey());
	}

	@Test
	public void testGetEndpointForDidWorksForUnknownDid() throws Exception {
		thrown.expect(ExecutionException.class);
		thrown.expectCause(isA(WalletValueNotFoundException.class));

		Signus.getEndpointForDid(wallet, DID).get();
	}
}