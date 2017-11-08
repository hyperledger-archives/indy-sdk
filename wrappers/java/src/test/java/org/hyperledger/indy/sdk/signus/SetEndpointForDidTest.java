package org.hyperledger.indy.sdk.signus;

import org.hyperledger.indy.sdk.IndyIntegrationTestWithPoolAndSingleWallet;
import org.hyperledger.indy.sdk.InvalidStructureException;
import org.junit.Test;

import java.util.concurrent.ExecutionException;

import static org.hamcrest.CoreMatchers.isA;
import static org.junit.Assert.assertEquals;


public class SetEndpointForDidTest extends IndyIntegrationTestWithPoolAndSingleWallet {

	@Test
	public void testSetEndpointForDidWorks() throws Exception {
		Signus.setEndpointForDid(wallet, DID, ENDPOINT, VERKEY).get();
	}

	@Test
	public void testSetEndpointForDidWorksForReplace() throws Exception {
		Signus.setEndpointForDid(wallet, DID, ENDPOINT, VERKEY).get();
		SignusResults.EndpointForDidResult receivedEndpoint = Signus.getEndpointForDid(wallet, pool, DID).get();
		assertEquals(ENDPOINT, receivedEndpoint.getAddress());
		assertEquals(VERKEY, receivedEndpoint.getTransportKey());

		String newEndpoin = "10.10.10.1:9710";
		Signus.setEndpointForDid(wallet, DID, newEndpoin, VERKEY_MY2).get();
		SignusResults.EndpointForDidResult updatedEndpoint = Signus.getEndpointForDid(wallet, pool, DID).get();
		assertEquals(newEndpoin, updatedEndpoint.getAddress());
		assertEquals(VERKEY_MY2, updatedEndpoint.getTransportKey());
	}

	@Test
	public void testSetEndpointForDidWorksForInvalidDid() throws Exception {
		thrown.expect(ExecutionException.class);
		thrown.expectCause(isA(InvalidStructureException.class));

		Signus.setEndpointForDid(wallet, INVALID_DID, ENDPOINT, VERKEY).get();
	}

	@Test
	public void testSetEndpointForDidWorksForInvalidTransportKey() throws Exception {
		thrown.expect(ExecutionException.class);
		thrown.expectCause(isA(InvalidStructureException.class));

		Signus.setEndpointForDid(wallet, DID, ENDPOINT, INVALID_VERKEY).get();
	}
}