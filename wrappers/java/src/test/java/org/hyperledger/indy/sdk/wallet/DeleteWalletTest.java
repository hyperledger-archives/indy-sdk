package org.hyperledger.indy.sdk.wallet;

import org.hyperledger.indy.sdk.ErrorCode;
import org.hyperledger.indy.sdk.ErrorCodeMatcher;
import org.hyperledger.indy.sdk.IndyIntegrationTest;

import static org.junit.Assert.assertNotNull;

import org.junit.Ignore;
import org.junit.Test;

import java.util.concurrent.ExecutionException;


public class DeleteWalletTest extends IndyIntegrationTest {

	@Test
	public void testDeleteWalletWorks() throws Exception {

		String poolName = "default";
		String walletName = "deleteWalletWorks";
		String type = "default";

		Wallet.createWallet(poolName, walletName, type, null, null).get();
		Wallet.deleteWallet(walletName, null).get();
		Wallet.createWallet(poolName, walletName, type, null, null).get();
	}

	@Test
	public void testDeleteWalletWorksForClosed() throws Exception {

		String poolName = "default";
		String walletName = "deleteWalletWorksForOpened";

		Wallet.createWallet(poolName, walletName, null, null, null).get();

		Wallet wallet = Wallet.openWallet(walletName, null, null).get();
		assertNotNull(wallet);

		wallet.closeWallet().get();
		Wallet.deleteWallet(walletName, null).get();
		Wallet.createWallet(poolName, walletName, null, null, null).get();
	}

	@Test
	@Ignore//TODO THERE IS BUG IN INDY
	public void testDeleteWalletWorksForOpened() throws Exception {

		thrown.expect(ExecutionException.class);
		thrown.expectCause(new ErrorCodeMatcher(ErrorCode.CommonIOError));

		String walletName = "deleteWalletWorksForOpened";

		Wallet.createWallet("default", walletName, null, null, null).get();
		Wallet.openWallet(walletName, null, null).get();
		Wallet.deleteWallet(walletName, null).get();
	}

	@Test
	public void testDeleteWalletWorksForTwice() throws Exception {

		thrown.expect(ExecutionException.class);
		thrown.expectCause(new ErrorCodeMatcher(ErrorCode.CommonIOError));

		String walletName = "deleteWalletWorksForTwice";

		Wallet.createWallet("default", walletName, null, null, null).get();

		Wallet wallet = Wallet.openWallet(walletName, null, null).get();
		assertNotNull(wallet);

		wallet.closeWallet().get();

		Wallet.deleteWallet(walletName, null).get();
		Wallet.deleteWallet(walletName, null).get();
	}

	@Test
	public void testDeleteWalletWorksForPlugged() throws Exception {
		String type = "inmem";
		String poolName = "default";
		String walletName = "wallet";

		Wallet.createWallet(poolName, walletName, type, null, null).get();
		Wallet.deleteWallet(walletName, null).get();
		Wallet.createWallet(poolName, walletName, type, null, null).get();
	}

	@Test
	public void testDeleteWalletWorksForNotCreated() throws Exception {

		thrown.expect(ExecutionException.class);
		thrown.expectCause(new ErrorCodeMatcher(ErrorCode.CommonIOError));

		Wallet.deleteWallet("deleteWalletWorksForTwice", null).get();
	}
}
