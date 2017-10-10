package org.hyperledger.indy.sdk.signus;

import org.hyperledger.indy.sdk.ErrorCode;
import org.hyperledger.indy.sdk.ErrorCodeMatcher;
import org.hyperledger.indy.sdk.IndyIntegrationTestWithSingleWallet;
import org.junit.Test;

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
		thrown.expectCause(new ErrorCodeMatcher(ErrorCode.CommonInvalidStructure));

		Signus.storeTheirDid(this.wallet, "{\"field\":\"value\"}").get();
	}

	@Test
	public void testStoreTheirDidWorksWithVerkey() throws Exception {
		Signus.storeTheirDid(this.wallet, String.format(IDENTITY_JSON_TEMPLATE, DID1, verkey)).get();
	}

	@Test
	public void testStoreTheirDidWorksWithoutDid() throws Exception {
		thrown.expect(ExecutionException.class);
		thrown.expectCause(new ErrorCodeMatcher(ErrorCode.CommonInvalidStructure));

		Signus.storeTheirDid(this.wallet, String.format("{\"verkey\":\"%s\"}", verkey)).get();
	}

	@Test
	public void testStoreTheirDidWorksForCorrectCryptoType() throws Exception {
		Signus.storeTheirDid(this.wallet, String.format("{\"did\":\"%s\", \"verkey\":\"%s\", \"crypto_type\": \"ed25519\"}", DID1, verkey)).get();
	}

	@Test
	public void testStoreTheirDidWorksForInvalidCryptoType() throws Exception {
		thrown.expect(ExecutionException.class);
		thrown.expectCause(new ErrorCodeMatcher(ErrorCode.SignusUnknownCryptoError));

		Signus.storeTheirDid(this.wallet, String.format("{\"did\":\"%s\", \"verkey\":\"%s\", \"crypto_type\": \"some_type\"}", DID1, verkey)).get();
	}
}
