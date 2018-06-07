package org.hyperledger.indy.sdk.wallet;

import org.hyperledger.indy.sdk.IOException;
import org.hyperledger.indy.sdk.IndyIntegrationTest;

import static org.hamcrest.CoreMatchers.isA;
import static org.junit.Assert.assertNotNull;

import org.junit.Ignore;
import org.junit.Test;

import java.util.concurrent.ExecutionException;


public class OpenWalletTest extends IndyIntegrationTest {

	@Test
	public void testOpenWalletWorks() throws Exception {
		Wallet.createWallet(POOL, "walletOpen", TYPE, null, CREDENTIALS).get();

		Wallet wallet = Wallet.openWallet("walletOpen", null, CREDENTIALS).get();
		assertNotNull(wallet);
	}

	@Test
	public void testOpenWalletWorksForConfig() throws Exception {
		Wallet.createWallet(POOL, "openWalletWorksForConfig", TYPE, null, CREDENTIALS).get();

		Wallet wallet = Wallet.openWallet("openWalletWorksForConfig", "{\"freshness_time\":1000}", CREDENTIALS).get();
		assertNotNull(wallet);
	}

	@Test
	public void testOpenWalletWorksForInvalidCredentials() throws Exception {
		Wallet.createWallet(POOL, "ForEbcryptedWalletInvalidCredentials", TYPE, null, CREDENTIALS).get();

		thrown.expect(ExecutionException.class);
		thrown.expectCause(isA(WalletAccessFailedException.class));

		Wallet.openWallet("ForEbcryptedWalletInvalidCredentials", null, "{\"key\": \"other_key\"}").get();
	}


	@Test
	public void testOpenWalletWorksForEbcryptedWalletChangingCredentials() throws Exception {
		Wallet.createWallet(POOL, "ForEbcryptedWalletChangingCredentials", TYPE, null, CREDENTIALS).get();

		Wallet wallet = Wallet.openWallet("ForEbcryptedWalletChangingCredentials", null, "{\"key\": \"key\", \"rekey\": \"other_key\"}").get();
		wallet.closeWallet().get();

		wallet = Wallet.openWallet("ForEbcryptedWalletChangingCredentials", null, "{\"key\": \"other_key\"}").get();
		wallet.closeWallet().get();
	}

	@Test
	@Ignore
	public void testOpenWalletWorksForPlugged() throws Exception {
		Wallet.createWallet(POOL, "testOpenWalletWorksForPlugged", "inmem", null, CREDENTIALS).get();
		Wallet wallet = Wallet.openWallet("testOpenWalletWorksForPlugged", null, CREDENTIALS).get();
		assertNotNull(wallet);
	}

	@Test
	public void testOpenWalletWorksForNotCreatedWallet() throws Exception {
		thrown.expect(ExecutionException.class);
		thrown.expectCause(isA(WalletNotFoundException.class));

		Wallet.openWallet("openWalletWorksForNotCreatedWallet", null, CREDENTIALS).get();
	}

	@Test
	public void testOpenWalletWorksForTwice() throws Exception {
		thrown.expect(ExecutionException.class);
		thrown.expectCause(isA(WalletAlreadyOpenedException.class));

		Wallet.createWallet(POOL, "openWalletWorksForTwice", TYPE, null, CREDENTIALS).get();

		Wallet.openWallet("openWalletWorksForTwice", null, CREDENTIALS).get();
		Wallet.openWallet("openWalletWorksForTwice", null, CREDENTIALS).get();
	}
}
