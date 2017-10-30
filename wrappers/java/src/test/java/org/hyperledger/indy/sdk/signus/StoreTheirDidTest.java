package org.hyperledger.indy.sdk.signus;

import org.hyperledger.indy.sdk.IndyIntegrationTestWithSingleWallet;
import org.hyperledger.indy.sdk.InvalidStructureException;
import org.junit.Test;

import static org.hamcrest.CoreMatchers.isA;

import java.util.concurrent.ExecutionException;

public class StoreTheirDidTest extends IndyIntegrationTestWithSingleWallet {

	private String verkey = "GjZWsBLgZCR18aL468JAT7w9CZRiBnpxUPPgyQxh4voa";

	@Test
	public void testStoreTheirDidWorks() throws Exception {
		Signus.storeTheirDid(this.wallet, String.format("{\"did\":\"%s\"}", DID1)).get();
	}

	@Test
	public void testCreateMyDidWorksForInvalidIdentityJson() throws Exception {
		thrown.expect(ExecutionException.class);
		thrown.expectCause(isA(InvalidStructureException.class));

		Signus.storeTheirDid(this.wallet, "{\"field\":\"value\"}").get();
	}

	@Test
	public void testStoreTheirDidWorksWithVerkey() throws Exception {
		Signus.storeTheirDid(this.wallet, String.format(IDENTITY_JSON_TEMPLATE, DID1, verkey)).get();
	}

	@Test
	public void testStoreTheirDidWorksWithoutDid() throws Exception {
		thrown.expect(ExecutionException.class);
		thrown.expectCause(isA(InvalidStructureException.class));

		Signus.storeTheirDid(this.wallet, String.format("{\"verkey\":\"%s\"}", verkey)).get();
	}
}
