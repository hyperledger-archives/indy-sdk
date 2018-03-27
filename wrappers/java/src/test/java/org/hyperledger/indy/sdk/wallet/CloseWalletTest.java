package org.hyperledger.indy.sdk.wallet;

import org.hyperledger.indy.sdk.IndyIntegrationTest;

import org.junit.Test;

import static org.hamcrest.CoreMatchers.isA;
import static org.junit.Assert.assertNotNull;

import java.util.concurrent.ExecutionException;


public class CloseWalletTest extends IndyIntegrationTest {

	@Test
	public void testCloseWalletWorks() throws Exception {
		Wallet.createWallet(POOL, WALLET, TYPE, null, null).get();
		Wallet wallet = Wallet.openWallet(WALLET, null, null).get();

		wallet.closeWallet().get();
	}

	@Test
	public void testCloseWalletWorksForTwice() throws Exception {
		thrown.expect(ExecutionException.class);
		thrown.expectCause(isA(InvalidWalletException.class));

		Wallet.createWallet(POOL, WALLET, TYPE, null, null).get();
		Wallet wallet = Wallet.openWallet(WALLET, null, null).get();

		wallet.closeWallet().get();
		wallet.closeWallet().get();
	}

	@Test
	public void testCloseWalletWorksForPlugged() throws Exception {
		Wallet.createWallet(POOL, WALLET, "inmem", null, null).get();

		Wallet wallet = Wallet.openWallet(WALLET, null, null).get();
		wallet.closeWallet().get();
	}

	@Test
	public void testAutoCloseWorks() throws Exception {
		Wallet.createWallet(POOL, WALLET, TYPE, null, null);
		try (Wallet wallet = Wallet.openWallet(WALLET, null, null).get()) {
			assertNotNull(wallet);
		}
		Wallet wallet = Wallet.openWallet(WALLET, null, null).get();
		wallet.closeWallet().get();
	}
}