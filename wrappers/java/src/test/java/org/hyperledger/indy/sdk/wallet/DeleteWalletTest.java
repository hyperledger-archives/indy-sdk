package org.hyperledger.indy.sdk.wallet;

import org.hyperledger.indy.sdk.IOException;
import org.hyperledger.indy.sdk.IndyIntegrationTest;

import static org.hamcrest.CoreMatchers.isA;
import static org.junit.Assert.assertNotNull;

import org.junit.Ignore;
import org.junit.Test;

import java.util.concurrent.ExecutionException;


public class DeleteWalletTest extends IndyIntegrationTest {

	@Test
	public void testDeleteWalletWorks() throws Exception {
		Wallet.createWallet(POOL, WALLET, TYPE, null, CREDENTIALS).get();
		Wallet.deleteWallet(WALLET, CREDENTIALS).get();
		Wallet.createWallet(POOL, WALLET, TYPE, null, CREDENTIALS).get();
		Wallet.deleteWallet(WALLET, CREDENTIALS).get();
	}

	@Test
	public void testDeleteWalletWorksForClosed() throws Exception {
		Wallet.createWallet(POOL, WALLET, null, null, CREDENTIALS).get();

		Wallet wallet = Wallet.openWallet(WALLET, null, CREDENTIALS).get();
		assertNotNull(wallet);

		wallet.closeWallet().get();
		Wallet.deleteWallet(WALLET, CREDENTIALS).get();
		Wallet.createWallet(POOL, WALLET, null, null, CREDENTIALS).get();
		Wallet.deleteWallet(WALLET, CREDENTIALS).get();
	}

	@Test
	@Ignore//TODO THERE IS BUG IN INDY
	public void testDeleteWalletWorksForOpened() throws Exception {
		thrown.expect(ExecutionException.class);
		thrown.expectCause(isA(IOException.class));

		String walletName = "deleteWalletWorksForOpened";

		Wallet.createWallet(POOL, walletName, null, null, CREDENTIALS).get();
		Wallet.openWallet(walletName, null, CREDENTIALS).get();
		Wallet.deleteWallet(walletName, CREDENTIALS).get();
	}

	@Test
	public void testDeleteWalletWorksForTwice() throws Exception {
		thrown.expect(ExecutionException.class);
		thrown.expectCause(isA(WalletNotFoundException.class));

		Wallet.createWallet(POOL, WALLET, null, null, CREDENTIALS).get();

		Wallet wallet = Wallet.openWallet(WALLET, null, CREDENTIALS).get();

		wallet.closeWallet().get();

		Wallet.deleteWallet(WALLET, CREDENTIALS).get();
		Wallet.deleteWallet(WALLET, CREDENTIALS).get();
	}

	@Test
	@Ignore
	public void testDeleteWalletWorksForPlugged() throws Exception {
		Wallet.createWallet(POOL, "pluggedWalletDelete", "inmem", null, CREDENTIALS).get();
		Wallet.deleteWallet("pluggedWalletDelete", CREDENTIALS).get();
		Wallet.createWallet(POOL, "pluggedWalletDelete", "inmem", null, CREDENTIALS).get();
	}

	@Test
	public void testDeleteWalletWorksForNotCreated() throws Exception {
		thrown.expect(ExecutionException.class);
		thrown.expectCause(isA(WalletNotFoundException.class));

		Wallet.deleteWallet(WALLET, CREDENTIALS).get();
	}
}
