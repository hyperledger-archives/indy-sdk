package org.hyperledger.indy.sdk.signus;

import org.hyperledger.indy.sdk.IndyIntegrationTestWithPoolAndSingleWallet;
import org.junit.Test;


import static junit.framework.TestCase.assertFalse;
import static org.junit.Assert.assertTrue;

public class VerifyTest extends IndyIntegrationTestWithPoolAndSingleWallet {

	@Test
	public void testVerifyWorksForVerkeyCachedInWallet() throws Exception {
		String identityJson = String.format(IDENTITY_JSON_TEMPLATE, DID_TRUSTEE, VERKEY_TRUSTEE);
		Signus.storeTheirDid(wallet, identityJson).get();

		Boolean valid = Signus.verifySignature(wallet, pool, DID_TRUSTEE, MESSAGE, SIGNATURE).get();
		assertTrue(valid);
	}

	@Test
	public void testVerifyWorksForGetNymFromLedger() throws Exception {
		Boolean valid = Signus.verifySignature(wallet, pool, DID_TRUSTEE, MESSAGE, SIGNATURE).get();
		assertTrue(valid);
	}

	@Test
	public void testVerifyWorksForOtherSigner() throws Exception {
		String identityJson = String.format(IDENTITY_JSON_TEMPLATE, DID_MY1, VERKEY_MY1);
		Signus.storeTheirDid(wallet, identityJson).get();

		Boolean valid = Signus.verifySignature(wallet, pool, DID_MY1, MESSAGE, SIGNATURE).get();
		assertFalse(valid);
	}
}
