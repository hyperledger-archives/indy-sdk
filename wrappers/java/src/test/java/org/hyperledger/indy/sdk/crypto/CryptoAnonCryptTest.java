package org.hyperledger.indy.sdk.crypto;

import org.hyperledger.indy.sdk.IndyIntegrationTest;
import org.hyperledger.indy.sdk.InvalidStructureException;
import org.junit.Test;

import java.util.concurrent.ExecutionException;

import static org.hamcrest.CoreMatchers.isA;
import static org.junit.Assert.assertNotNull;

public class CryptoAnonCryptTest extends IndyIntegrationTest {

	@Test
	public void testPrepAnonymousMsgWorks() throws Exception {
		byte[] encryptedMsg = Crypto.anonCrypt(VERKEY_MY1, MESSAGE).get();
		assertNotNull(encryptedMsg);
	}

	@Test
	public void testPrepAnonymousMsgWorksForInvalidRecipientVk() throws Exception {
		thrown.expect(ExecutionException.class);
		thrown.expectCause(isA(InvalidStructureException.class));

		Crypto.anonCrypt(INVALID_VERKEY, MESSAGE).get();
	}
}