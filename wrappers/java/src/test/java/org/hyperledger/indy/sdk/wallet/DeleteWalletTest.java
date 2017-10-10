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
		Wallet.createWallet(POOL, WALLET, TYPE, null, null).get();
		Wallet.deleteWallet(WALLET, null).get();
		Wallet.createWallet(POOL, WALLET, TYPE, null, null).get();
		Wallet.deleteWallet(WALLET, null).get();
	}

	@Test
	public void testDeleteWalletWorksForClosed() throws Exception {
		Wallet.createWallet(POOL, WALLET, null, null, null).get();

		Wallet wallet = Wallet.openWallet(WALLET, null, null).get();
		assertNotNull(wallet);

		wallet.closeWallet().get();
		Wallet.deleteWallet(WALLET, null).get();
		Wallet.createWallet(POOL, WALLET, null, null, null).get();
		Wallet.deleteWallet(WALLET, null).get();
	}

	@Test
	@Ignore//TODO THERE IS BUG IN INDY
	public void testDeleteWalletWorksForOpened() throws Exception {
		thrown.expect(ExecutionException.class);
		thrown.expectCause(new ErrorCodeMatcher(ErrorCode.CommonIOError));

		String walletName = "deleteWalletWorksForOpened";

		Wallet.createWallet(POOL, walletName, null, null, null).get();
		Wallet.openWallet(walletName, null, null).get();
		Wallet.deleteWallet(walletName, null).get();
	}

	@Test
	public void testDeleteWalletWorksForTwice() throws Exception {
		thrown.expect(ExecutionException.class);
		thrown.expectCause(new ErrorCodeMatcher(ErrorCode.CommonIOError));

		Wallet.createWallet(POOL, WALLET, null, null, null).get();

		Wallet wallet = Wallet.openWallet(WALLET, null, null).get();

		wallet.closeWallet().get();

		Wallet.deleteWallet(WALLET, null).get();
		Wallet.deleteWallet(WALLET, null).get();
	}

	@Test
	public void testDeleteWalletWorksForPlugged() throws Exception {
		Wallet.createWallet(POOL, "pluggedWalletDelete", "inmem", null, null).get();
		Wallet.deleteWallet("pluggedWalletDelete", null).get();
		Wallet.createWallet(POOL, "pluggedWalletDelete", "inmem", null, null).get();
	}

	@Test
	public void testDeleteWalletWorksForNotCreated() throws Exception {
		thrown.expect(ExecutionException.class);
		thrown.expectCause(new ErrorCodeMatcher(ErrorCode.CommonIOError));

		Wallet.deleteWallet(WALLET, null).get();
	}
}
