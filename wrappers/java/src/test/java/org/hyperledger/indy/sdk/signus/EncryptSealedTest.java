package org.hyperledger.indy.sdk.signus;

import org.hyperledger.indy.sdk.IndyIntegrationTestWithPoolAndSingleWallet;
import org.hyperledger.indy.sdk.InvalidStateException;
import org.junit.Test;

import java.util.concurrent.ExecutionException;

import static org.hamcrest.CoreMatchers.isA;
import static org.junit.Assert.assertNotNull;

public class EncryptSealedTest extends IndyIntegrationTestWithPoolAndSingleWallet {

	@Test
	public void testEncryptSealedWorksForPkCachedInWallet() throws Exception {
		String identityJson = String.format(IDENTITY_JSON_TEMPLATE, DID_TRUSTEE, VERKEY_TRUSTEE);
		Signus.storeTheirDid(wallet, identityJson).get();

		byte[] encryptResult = Signus.encryptSealed(wallet, pool, DID_TRUSTEE, MESSAGE).get();
		assertNotNull(encryptResult);
	}

	@Test
	public void testEncryptSealedWorksForGetNymFromLedger() throws Exception {
		byte[] encryptResult = Signus.encryptSealed(wallet, pool, DID_TRUSTEE, MESSAGE).get();
		assertNotNull(encryptResult);
	}

	@Test
	public void testEncryptSealedWorksForNotFoundNym() throws Exception {
		thrown.expect(ExecutionException.class);
		thrown.expectCause(isA(InvalidStateException.class));
		
		Signus.encryptSealed(wallet, pool, DID_MY2, MESSAGE).get();
	}
}
