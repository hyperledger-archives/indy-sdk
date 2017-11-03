package org.hyperledger.indy.sdk.signus;

import org.hyperledger.indy.sdk.IndyIntegrationTestWithSingleWallet;
import org.hyperledger.indy.sdk.InvalidStructureException;
import org.junit.Test;

import static org.hamcrest.CoreMatchers.isA;

import java.util.concurrent.ExecutionException;

public class StoreTheirDidTest extends IndyIntegrationTestWithSingleWallet {

	@Test
	public void testStoreTheirDidWorks() throws Exception {
		Signus.storeTheirDid(this.wallet, String.format("{\"did\":\"%s\"}", DID)).get();
	}

	@Test
	public void testCreateMyDidWorksForInvalidIdentityJson() throws Exception {
		thrown.expect(ExecutionException.class);
		thrown.expectCause(isA(InvalidStructureException.class));

		Signus.storeTheirDid(this.wallet, "{\"field\":\"value\"}").get();
	}

	@Test
	public void testStoreTheirDidWorksWithVerkey() throws Exception {
		Signus.storeTheirDid(this.wallet, String.format(IDENTITY_JSON_TEMPLATE, DID, VERKEY)).get();
	}

	@Test
	public void testStoreTheirDidWorksWithoutDid() throws Exception {
		thrown.expect(ExecutionException.class);
		thrown.expectCause(isA(InvalidStructureException.class));

		Signus.storeTheirDid(this.wallet, String.format("{\"verkey\":\"%s\"}", VERKEY)).get();
	}
}
