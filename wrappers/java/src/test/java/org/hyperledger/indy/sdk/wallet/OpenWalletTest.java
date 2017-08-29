package org.hyperledger.indy.sdk.wallet;

import org.hyperledger.indy.sdk.ErrorCode;
import org.hyperledger.indy.sdk.ErrorCodeMatcher;
import org.hyperledger.indy.sdk.IndyIntegrationTest;

import static org.junit.Assert.assertNotNull;

import org.junit.Test;

import java.util.concurrent.ExecutionException;


public class OpenWalletTest extends IndyIntegrationTest {

	@Test
	public void testOpenWalletWorks() throws Exception {

		String walletName = "deleteWalletWorks";

		Wallet.createWallet("default", walletName, "default", null, null).get();

		Wallet wallet = Wallet.openWallet(walletName, null, null).get();
		assertNotNull(wallet);
	}

	@Test
	public void testOpenWalletWorksForConfig() throws Exception {

		String walletName = "openWalletWorksForConfig";

		Wallet.createWallet("default", walletName, "default", null, null).get();

		Wallet wallet = Wallet.openWallet(walletName, "{\"freshness_time\":1000}", null).get();
		assertNotNull(wallet);
	}

	@Test
	public void testOpenWalletWorksForPlugged() throws Exception {
		String type = "inmem";
		String poolName = "default";
		String walletName = "testOpenWalletWorksForPlugged";

		Wallet.createWallet(poolName, walletName, type, null, null).get();
		Wallet wallet = Wallet.openWallet(walletName, null, null).get();
		assertNotNull(wallet);
	}

	@Test
	public void testOpenWalletWorksForNotCreatedWallet() throws Exception {

		thrown.expect(ExecutionException.class);
		thrown.expectCause(new ErrorCodeMatcher(ErrorCode.CommonIOError));

		Wallet.openWallet("openWalletWorksForNotCreatedWallet", null, null).get();
	}

	@Test
	public void testOpenWalletWorksForTwice() throws Exception {

		thrown.expect(ExecutionException.class);
		thrown.expectCause(new ErrorCodeMatcher(ErrorCode.WalletAlreadyOpenedError));

		String walletName = "openWalletWorksForTwice";

		Wallet.createWallet("default", walletName, "default", null, null).get();

		Wallet.openWallet(walletName, null, null).get();
		Wallet.openWallet(walletName, null, null).get();
	}
}
