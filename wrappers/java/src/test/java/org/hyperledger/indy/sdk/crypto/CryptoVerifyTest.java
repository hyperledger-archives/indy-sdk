package org.hyperledger.indy.sdk.crypto;

import org.hyperledger.indy.sdk.IndyIntegrationTest;
import org.junit.Test;

import static org.junit.Assert.assertTrue;

public class CryptoVerifyTest extends IndyIntegrationTest {

	@Test
	public void testCryptoVerifyWorks() throws Exception {
		Boolean valid = Crypto.cryptoVerify(VERKEY_TRUSTEE, MESSAGE, SIGNATURE).get();
		assertTrue(valid);
	}
}
