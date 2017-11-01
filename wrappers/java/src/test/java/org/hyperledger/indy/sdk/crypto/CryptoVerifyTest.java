package org.hyperledger.indy.sdk.crypto;

import org.hyperledger.indy.sdk.IndyIntegrationTest;
import org.junit.Test;

import java.util.concurrent.ExecutionException;

import static junit.framework.TestCase.assertFalse;
import static org.hamcrest.CoreMatchers.isA;
import static org.junit.Assert.assertTrue;

public class CryptoVerifyTest extends IndyIntegrationTest {

	@Test
	public void testCryptoVerifyWorks() throws Exception {
		Boolean valid = Crypto.cryptoVerify(VERKEY_TRUSTEE, MESSAGE, SIGNATURE).get();
		assertTrue(valid);
	}

	@Test
	public void testCryptoVerifyWorksForVerkeyWithCorrectCryptoType() throws Exception {
		String verkey = VERKEY_TRUSTEE + ":ed25519";
		Boolean valid = Crypto.cryptoVerify(verkey, MESSAGE, SIGNATURE).get();
		assertTrue(valid);
	}

	@Test
	public void testCryptoVerifyWorksForVerkeyWithInvalidCryptoType() throws Exception {
		thrown.expect(ExecutionException.class);
		thrown.expectCause(isA(UnknownCryptoException.class));

		String verkey = VERKEY_TRUSTEE + ":unknown_crypto";
		Crypto.cryptoVerify(verkey, MESSAGE, SIGNATURE).get();
	}

	@Test
	public void testCryptoVerifyWorksForOtherSigner() throws Exception {
		Boolean valid = Crypto.cryptoVerify(VERKEY_MY2, MESSAGE, SIGNATURE).get();
		assertFalse(valid);
	}
}
