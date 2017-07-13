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

	@Before
	public void createWallet() throws Exception {
		Wallet.createWallet("default", "signusWallet", "default", null, null).get();
		this.wallet = Wallet.openWallet("signusWallet", null, null).get();

	}

	@After
	public void deleteWallet() throws Exception {

		this.wallet.closeWallet();
		Wallet.deleteWallet("signusWallet", null);
	}

	@Test
	public void testStoreTheirDidWorks() throws Exception {
		Signus.storeTheirDid(this.wallet, "{\"did\":\"8wZcEriaNLNKtteJvx7f8i\"}").get();
	}

	@Test
	public void testCreateMyDidWorksForInvalidIdentityJson() throws Exception {
		thrown.expect(ExecutionException.class);
		thrown.expectCause(new ErrorCodeMatcher(ErrorCode.CommonInvalidStructure));

		Signus.storeTheirDid(this.wallet, "{\"field\":\"value\"}").get();
	}

	@Test
	public void testStoreTheirDidWorksWithVerkey() throws Exception {
		Signus.storeTheirDid(this.wallet,"{\"did\":\"8wZcEriaNLNKtteJvx7f8i\", " +
				"\"verkey\":\"GjZWsBLgZCR18aL468JAT7w9CZRiBnpxUPPgyQxh4voa\"}").get();
	}

	@Test
	public void testStoreTheirDidWorksWithoutDid() throws Exception {
		thrown.expect(ExecutionException.class);
		thrown.expectCause(new ErrorCodeMatcher(ErrorCode.CommonInvalidStructure));

		Signus.storeTheirDid(this.wallet, "{\"verkey\":\"GjZWsBLgZCR18aL468JAT7w9CZRiBnpxUPPgyQxh4voa\"}").get();
	}

	@Test
	public void testStoreTheirDidWorksForCorrectCryptoType() throws Exception {
		Signus.storeTheirDid(this.wallet, "{\"did\":\"8wZcEriaNLNKtteJvx7f8i\", " +
				"\"verkey\":\"GjZWsBLgZCR18aL468JAT7w9CZRiBnpxUPPgyQxh4voa\", \"crypto_type\": \"ed25519\"}").get();
	}

	@Test
	public void testStoreTheirDidWorksForInvalidCryptoType() throws Exception {
		thrown.expect(ExecutionException.class);
		thrown.expectCause(new ErrorCodeMatcher(ErrorCode.SignusUnknownCryptoError));

		Signus.storeTheirDid(this.wallet, "{\"did\":\"8wZcEriaNLNKtteJvx7f8i\", " +
				"\"verkey\":\"GjZWsBLgZCR18aL468JAT7w9CZRiBnpxUPPgyQxh4voa\", \"crypto_type\": \"some_type\"}").get();
	}
}
