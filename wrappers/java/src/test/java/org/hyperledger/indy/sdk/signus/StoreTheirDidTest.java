package org.hyperledger.indy.sdk.signus;

import org.hyperledger.indy.sdk.ErrorCode;
import org.hyperledger.indy.sdk.ErrorCodeMatcher;
import org.hyperledger.indy.sdk.IndyIntegrationTest;
import org.hyperledger.indy.sdk.wallet.Wallet;
import org.junit.After;
import org.junit.Before;
import org.junit.Test;

import java.util.concurrent.ExecutionException;

public class StoreTheirDidTest extends IndyIntegrationTest {


	private Wallet wallet;
	private String walletName = "signusWallet";
	private String did = "8wZcEriaNLNKtteJvx7f8i";
	private String verkey = "GjZWsBLgZCR18aL468JAT7w9CZRiBnpxUPPgyQxh4voa";

	@Before
	public void createWallet() throws Exception {
		Wallet.createWallet("default", walletName, "default", null, null).get();
		this.wallet = Wallet.openWallet(walletName, null, null).get();

	}

	@After
	public void deleteWallet() throws Exception {

		this.wallet.closeWallet().get();
		Wallet.deleteWallet(walletName, null).get();
	}

	@Test
	public void testStoreTheirDidWorks() throws Exception {
		Signus.storeTheirDid(this.wallet, String.format("{\"did\":\"%s\"}", did)).get();
	}

	@Test
	public void testCreateMyDidWorksForInvalidIdentityJson() throws Exception {
		thrown.expect(ExecutionException.class);
		thrown.expectCause(new ErrorCodeMatcher(ErrorCode.CommonInvalidStructure));

		Signus.storeTheirDid(this.wallet, "{\"field\":\"value\"}").get();
	}

	@Test
	public void testStoreTheirDidWorksWithVerkey() throws Exception {
		Signus.storeTheirDid(this.wallet, String.format("{\"did\":\"%s\", \"verkey\":\"%s\"}", did, verkey)).get();
	}

	@Test
	public void testStoreTheirDidWorksWithoutDid() throws Exception {
		thrown.expect(ExecutionException.class);
		thrown.expectCause(new ErrorCodeMatcher(ErrorCode.CommonInvalidStructure));

		Signus.storeTheirDid(this.wallet, String.format("{\"verkey\":\"%s\"}", verkey)).get();
	}

	@Test
	public void testStoreTheirDidWorksForCorrectCryptoType() throws Exception {
		Signus.storeTheirDid(this.wallet, String.format("{\"did\":\"%s\", \"verkey\":\"%s\", \"crypto_type\": \"ed25519\"}", did, verkey)).get();
	}

	@Test
	public void testStoreTheirDidWorksForInvalidCryptoType() throws Exception {
		thrown.expect(ExecutionException.class);
		thrown.expectCause(new ErrorCodeMatcher(ErrorCode.SignusUnknownCryptoError));

		Signus.storeTheirDid(this.wallet, String.format("{\"did\":\"%s\", \"verkey\":\"%s\", \"crypto_type\": \"some_type\"}", did, verkey)).get();
	}
}
