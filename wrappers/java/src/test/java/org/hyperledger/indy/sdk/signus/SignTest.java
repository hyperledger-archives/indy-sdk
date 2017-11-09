package org.hyperledger.indy.sdk.signus;

import org.hyperledger.indy.sdk.IndyIntegrationTestWithSingleWallet;
import org.hyperledger.indy.sdk.signus.SignusResults.CreateAndStoreMyDidResult;
import org.hyperledger.indy.sdk.wallet.WalletValueNotFoundException;

import static org.hamcrest.CoreMatchers.isA;
import static org.junit.Assert.assertTrue;

import org.junit.Test;

import java.util.Arrays;
import java.util.concurrent.ExecutionException;

public class SignTest extends IndyIntegrationTestWithSingleWallet {

	@Test
	public void testSignWorks() throws Exception {
		CreateAndStoreMyDidResult result = Signus.createAndStoreMyDid(this.wallet, TRUSTEE_IDENTITY_JSON).get();
		byte[] signature = Signus.sign(this.wallet, result.getDid(), MESSAGE).get();
		assertTrue(Arrays.equals(SIGNATURE, signature));
	}

	@Test
	public void testSignWorksForUnknowDid() throws Exception {
		thrown.expect(ExecutionException.class);
		thrown.expectCause(isA(WalletValueNotFoundException.class));

		Signus.sign(this.wallet, DID, MESSAGE).get();
	}
}
