package org.hyperledger.indy.sdk.signus;

import org.hyperledger.indy.sdk.IndyIntegrationTestWithSingleWallet;
import org.hyperledger.indy.sdk.InvalidStructureException;
import org.junit.Test;

import java.util.concurrent.ExecutionException;

import static org.hamcrest.CoreMatchers.isA;
import static org.junit.Assert.assertEquals;


public class SetEndpointForDidTest extends IndyIntegrationTestWithSingleWallet {

	@Test
	public void testSetEndpointForDidWorks() throws Exception {
		Signus.setEndpointForDid(wallet, DID1, ENDPOINT, VERKEY).get();
	}

	@Test
	public void testSetEndpointForDidWorksForReplace() throws Exception {
		Signus.setEndpointForDid(wallet, DID1, ENDPOINT, VERKEY).get();
		SignusResults.EndpointForDidResult receivedEndpoint = Signus.getEndpointForDid(wallet, DID1).get();
		assertEquals(ENDPOINT, receivedEndpoint.getAddress());
		assertEquals(VERKEY, receivedEndpoint.getTransportKey());

		String newEndpoin = "10.10.10.1:9710";
		Signus.setEndpointForDid(wallet, DID1, newEndpoin, VERKEY_FOR_MY2_SEED).get();
		SignusResults.EndpointForDidResult updatedEndpoint = Signus.getEndpointForDid(wallet, DID1).get();
		assertEquals(newEndpoin, updatedEndpoint.getAddress());
		assertEquals(VERKEY_FOR_MY2_SEED, updatedEndpoint.getTransportKey());
	}

	@Test
	public void testSetEndpointForDidWorksForInvalidDid() throws Exception {
		thrown.expect(ExecutionException.class);
		thrown.expectCause(isA(InvalidStructureException.class));

		Signus.setEndpointForDid(wallet, "invalid_base58string", ENDPOINT, VERKEY).get();
	}

	@Test
	public void testSetEndpointForDidWorksForInvalidTransportKey() throws Exception {
		thrown.expect(ExecutionException.class);
		thrown.expectCause(isA(InvalidStructureException.class));

		Signus.setEndpointForDid(wallet, DID1, ENDPOINT, INVALID_VERKEY).get();
	}
}