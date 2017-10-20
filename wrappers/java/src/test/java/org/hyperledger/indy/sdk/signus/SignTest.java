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

		CreateAndStoreMyDidResult result = Signus.createAndStoreMyDid(this.wallet, MY1_IDENTITY_JSON).get();

		byte[] expectedSignature = {- 87, - 41, 8, - 31, 7, 107, 110, 9, - 63, - 94, - 54, - 42, - 94, 66, - 18, - 45, 63, - 47, 12, - 60, 8, - 45, 55, 27, 120, 94,
				- 52, - 109, 53, 104, 103, 61, 60, - 7, - 19, 127, 103, 46, - 36, - 33, 10, 95, 75, 53, - 11, - 46, - 15, - 105, - 65, 41, 48, 30, 9, 16, 78, - 4,
				- 99, - 50, - 46, - 111, 125, - 123, 109, 11};

		byte[] signature = Signus.sign(this.wallet, result.getDid(), MESSAGE).get();

		assertTrue(Arrays.equals(expectedSignature, signature));
	}

	@Test
	public void testSignWorksForUnknowDid() throws Exception {
		thrown.expect(ExecutionException.class);
		thrown.expectCause(isA(WalletValueNotFoundException.class));

		Signus.sign(this.wallet, DID1, MESSAGE).get();
	}
}
