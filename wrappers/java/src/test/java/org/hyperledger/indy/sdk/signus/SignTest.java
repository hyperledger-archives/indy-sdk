package org.hyperledger.indy.sdk.signus;

import org.hyperledger.indy.sdk.ErrorCode;
import org.hyperledger.indy.sdk.ErrorCodeMatcher;
import org.hyperledger.indy.sdk.IndyIntegrationTest;
import org.hyperledger.indy.sdk.signus.SignusResults.CreateAndStoreMyDidResult;
import org.hyperledger.indy.sdk.wallet.Wallet;

import static org.junit.Assert.assertEquals;

import org.junit.After;
import org.junit.Before;
import org.junit.Test;

import java.util.concurrent.ExecutionException;

public class SignTest extends IndyIntegrationTest {


	private Wallet wallet;
	private String walletName = "signusWallet";

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
				new SignusJSONParameters.CreateAndStoreMyDidJSONParameter(null, TRUSTEE_SEED, null, null);

		CreateAndStoreMyDidResult result = Signus.createAndStoreMyDid(this.wallet, didJson.toJson()).get();
		String did = result.getDid();

		String msg = "{\"reqId\":1496822211362017764}";

		String expectedSignature = "R4Rj68n4HZosQqEc3oMUbQh7MtG8tH7WmXE2Mok8trHJ67CrzyqahZn5ziJy4nebRtq6Qi6fVH9JkvVCM85XjFa";

		String signedMessage = Signus.sign(this.wallet, did, msg).get();

		assertEquals(expectedSignature, signedMessage);
	}

	@Test
	public void testSignWorksForUnknowDid() throws Exception {
		thrown.expect(ExecutionException.class);
		thrown.expectCause(new ErrorCodeMatcher(ErrorCode.WalletNotFoundError));

		String msg = "{\"reqId\":1496822211362017764}";

		Signus.sign(this.wallet, "8wZcEriaNLNKtteJvx7f8i", msg).get();
	}
}
