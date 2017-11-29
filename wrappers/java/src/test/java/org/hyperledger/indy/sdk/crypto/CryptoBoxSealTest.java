package org.hyperledger.indy.sdk.crypto;

import org.hyperledger.indy.sdk.IndyIntegrationTest;
import org.hyperledger.indy.sdk.InvalidStructureException;
import org.junit.Test;

import java.util.concurrent.ExecutionException;

import static org.hamcrest.CoreMatchers.isA;

public class CryptoBoxSealTest extends IndyIntegrationTest {

	@Test
	public void testCryptoBoxSealWorks() throws Exception {
		Crypto.cryptoBoxSeal(VERKEY_MY1, MESSAGE).get();
	}

	@Test
	public void testCryptoBoxSealWorksForInvalidKey() throws Exception {
		thrown.expect(ExecutionException.class);
		thrown.expectCause(isA(InvalidStructureException.class));

		Crypto.cryptoBoxSeal(INVALID_VERKEY, MESSAGE).get();
	}
}
