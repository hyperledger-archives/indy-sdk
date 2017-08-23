package org.hyperledger.indy.sdk.wallet;

import org.hyperledger.indy.sdk.ErrorCode;
import org.hyperledger.indy.sdk.ErrorCodeMatcher;
import org.hyperledger.indy.sdk.IndyIntegrationTest;

import static org.junit.Assert.assertNotNull;

import org.junit.Test;

import java.util.concurrent.ExecutionException;


public class CloseWalletTest extends IndyIntegrationTest {

	@Test
	public void testCloseWalletWorks() throws Exception {

		String walletName = "closeWalletWorks";

		Wallet.createWallet("default", walletName, "default", null, null).get();

		Wallet wallet = Wallet.openWallet(walletName, null, null).get();
		assertNotNull(wallet);

		wallet.closeWallet().get();
	}

	@Test
	public void testCloseWalletWorksForTwice() throws Exception {

		thrown.expect(ExecutionException.class);
		thrown.expectCause(new ErrorCodeMatcher(ErrorCode.WalletInvalidHandle));

		String walletName = "closeWalletWorksForTwice";

		Wallet.createWallet("default", walletName, "default", null, null).get();

		Wallet wallet = Wallet.openWallet(walletName, null, null).get();
		assertNotNull(wallet);

		wallet.closeWallet().get();
		wallet.closeWallet().get();
	}

	@Test
	public void testCloseWalletWorksForPlugged() throws Exception {
		String walletName = "testCloseWalletWorksForPlugged";

		Wallet.createWallet("default", walletName, "inmem", null, null).get();

		Wallet wallet = Wallet.openWallet(walletName, null, null).get();
		wallet.closeWallet().get();
		Wallet.openWallet(walletName, null, null).get();
	}
}