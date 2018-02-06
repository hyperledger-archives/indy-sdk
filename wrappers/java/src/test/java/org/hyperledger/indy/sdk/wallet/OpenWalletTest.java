package org.hyperledger.indy.sdk.wallet;

import org.hyperledger.indy.sdk.IOException;
import org.hyperledger.indy.sdk.IndyIntegrationTest;

import static org.hamcrest.CoreMatchers.isA;
import static org.junit.Assert.assertNotNull;

import org.junit.Test;

import java.util.concurrent.ExecutionException;


public class OpenWalletTest extends IndyIntegrationTest {

	String credentials = "{\"key\":\"testkey\"}";

	@Test
	public void testOpenWalletWorks() throws Exception {
		Wallet.createWallet(POOL, "walletOpen", TYPE, null, null).get();

		Wallet wallet = Wallet.openWallet("walletOpen", null, null).get();
		assertNotNull(wallet);
	}

	@Test
	public void testOpenWalletWorksForConfig() throws Exception {
		Wallet.createWallet(POOL, "openWalletWorksForConfig", TYPE, null, null).get();

		Wallet wallet = Wallet.openWallet("openWalletWorksForConfig", "{\"freshness_time\":1000}", null).get();
		assertNotNull(wallet);
	}

	@Test
	public void testOpenWalletWorksForEbcryptedWalletCorrectCredentials() throws Exception {
		Wallet.createWallet(POOL, "ForEbcryptedWalletCorrectCredentials", TYPE, null, credentials).get();

		Wallet wallet = Wallet.openWallet("ForEbcryptedWalletCorrectCredentials", null, credentials).get();
		assertNotNull(wallet);
	}

	@Test
	public void testOpenWalletWorksForEbcryptedWalletInvalidCredentials() throws Exception {
		Wallet.createWallet(POOL, "ForEbcryptedWalletInvalidCredentials", TYPE, null, credentials).get();

		thrown.expect(ExecutionException.class);
		thrown.expectCause(isA(WalletAccessFailedException.class));

		Wallet.openWallet("ForEbcryptedWalletInvalidCredentials", null, "{\"key\":\"otherkey\"}").get();
	}


	@Test
	public void testOpenWalletWorksForEbcryptedWalletChangingCredentials() throws Exception {
		Wallet.createWallet(POOL, "ForEbcryptedWalletChangingCredentials", TYPE, null, credentials).get();

		Wallet wallet = Wallet.openWallet("ForEbcryptedWalletChangingCredentials", null, "{\"key\":\"testkey\", \"rekey\":\"otherkey\"}").get();
		assertNotNull(wallet);
	}

	@Test
	public void testOpenWalletWorksForPlugged() throws Exception {
		Wallet.createWallet(POOL, "testOpenWalletWorksForPlugged", "inmem", null, null).get();
		Wallet wallet = Wallet.openWallet("testOpenWalletWorksForPlugged", null, null).get();
		assertNotNull(wallet);
	}

	@Test
	public void testOpenWalletWorksForNotCreatedWallet() throws Exception {
		thrown.expect(ExecutionException.class);
		thrown.expectCause(isA(IOException.class));

		Wallet.openWallet("openWalletWorksForNotCreatedWallet", null, null).get();
	}

	@Test
	public void testOpenWalletWorksForTwice() throws Exception {
		thrown.expect(ExecutionException.class);
		thrown.expectCause(isA(WalletAlreadyOpenedException.class));

		Wallet.createWallet(POOL, "openWalletWorksForTwice", TYPE, null, null).get();

		Wallet.openWallet("openWalletWorksForTwice", null, null).get();
		Wallet.openWallet("openWalletWorksForTwice", null, null).get();
	}
}
