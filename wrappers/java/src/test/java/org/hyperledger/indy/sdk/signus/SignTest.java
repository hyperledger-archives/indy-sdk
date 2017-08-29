package org.hyperledger.indy.sdk.signus;

import org.hyperledger.indy.sdk.ErrorCode;
import org.hyperledger.indy.sdk.ErrorCodeMatcher;
import org.hyperledger.indy.sdk.IndyIntegrationTest;
import org.hyperledger.indy.sdk.signus.SignusResults.CreateAndStoreMyDidResult;
import org.hyperledger.indy.sdk.wallet.Wallet;

import static org.junit.Assert.assertTrue;

import org.junit.After;
import org.junit.Before;
import org.junit.Test;

import java.util.Arrays;
import java.util.concurrent.ExecutionException;

public class SignTest extends IndyIntegrationTest {


	private Wallet wallet;
	private String walletName = "signusWallet";
	private byte[] msg = "{\"reqId\":1496822211362017764}".getBytes();

	@Before
	public void createWalletWhitDid() throws Exception {
		Wallet.createWallet("default", walletName, "default", null, null).get();
		this.wallet = Wallet.openWallet(walletName, null, null).get();
	}

	@After
	public void deleteWallet() throws Exception {
		this.wallet.closeWallet().get();
		Wallet.deleteWallet(walletName, null).get();
	}

	@Test
	public void testSignWorks() throws Exception {

		SignusJSONParameters.CreateAndStoreMyDidJSONParameter didJson =
				new SignusJSONParameters.CreateAndStoreMyDidJSONParameter(null, "00000000000000000000000000000My1", null, null);

		CreateAndStoreMyDidResult result = Signus.createAndStoreMyDid(this.wallet, didJson.toJson()).get();

		byte[] expectedSignature = {- 87, - 41, 8, - 31, 7, 107, 110, 9, - 63, - 94, - 54, - 42, - 94, 66, - 18, - 45, 63, - 47, 12, - 60, 8, - 45, 55, 27, 120, 94,
				- 52, - 109, 53, 104, 103, 61, 60, - 7, - 19, 127, 103, 46, - 36, - 33, 10, 95, 75, 53, - 11, - 46, - 15, - 105, - 65, 41, 48, 30, 9, 16, 78, - 4,
				- 99, - 50, - 46, - 111, 125, - 123, 109, 11};

		byte[] signature = Signus.sign(this.wallet, result.getDid(), msg).get();

		assertTrue(Arrays.equals(expectedSignature, signature));
	}

	@Test
	public void testSignWorksForUnknowDid() throws Exception {
		thrown.expect(ExecutionException.class);
		thrown.expectCause(new ErrorCodeMatcher(ErrorCode.WalletNotFoundError));

		Signus.sign(this.wallet, "8wZcEriaNLNKtteJvx7f8i", msg).get();
	}
}
