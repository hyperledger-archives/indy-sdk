package org.hyperledger.indy.sdk.did;

import org.hyperledger.indy.sdk.IndyIntegrationTestWithPoolAndSingleWallet;
import org.hyperledger.indy.sdk.InvalidStructureException;
import org.junit.Test;

import java.util.concurrent.ExecutionException;

import static org.hamcrest.CoreMatchers.isA;
import static org.junit.Assert.assertEquals;


public class SetEndpointForDidTest extends IndyIntegrationTestWithPoolAndSingleWallet {

	@Test
	public void testSetEndpointForDidWorks() throws Exception {
		Did.setEndpointForDid(wallet, DID, ENDPOINT, VERKEY).get();
	}

	@Test
	public void testSetEndpointForDidWorksForReplace() throws Exception {
		Did.setEndpointForDid(wallet, DID, ENDPOINT, VERKEY).get();
		DidResults.EndpointForDidResult receivedEndpoint = Did.getEndpointForDid(wallet, pool, DID).get();
		assertEquals(ENDPOINT, receivedEndpoint.getAddress());
		assertEquals(VERKEY, receivedEndpoint.getTransportKey());

		String newEndpoint = "10.10.10.1:9710";
		Did.setEndpointForDid(wallet, DID, newEndpoint, VERKEY_MY2).get();
		DidResults.EndpointForDidResult updatedEndpoint = Did.getEndpointForDid(wallet, pool, DID).get();
		assertEquals(newEndpoint, updatedEndpoint.getAddress());
		assertEquals(VERKEY_MY2, updatedEndpoint.getTransportKey());
	}

	@Test
	public void testSetEndpointForDidWorksForInvalidDid() throws Exception {
		thrown.expect(ExecutionException.class);
		thrown.expectCause(isA(InvalidStructureException.class));

		Did.setEndpointForDid(wallet, INVALID_DID, ENDPOINT, VERKEY).get();
	}

	@Test
	public void testSetEndpointForDidWorksForInvalidTransportKey() throws Exception {
		thrown.expect(ExecutionException.class);
		thrown.expectCause(isA(InvalidStructureException.class));

		Did.setEndpointForDid(wallet, DID, ENDPOINT, INVALID_VERKEY).get();
	}
}