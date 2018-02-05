package org.hyperledger.indy.sdk.wallet;

import static org.hamcrest.CoreMatchers.isA;

import java.util.concurrent.ExecutionException;

import org.hyperledger.indy.sdk.IndyIntegrationTest;
import org.junit.Test;


public class CreateWalletTest extends IndyIntegrationTest {

	@Test
	public void testCreateWalletWorks() throws Exception {
		Wallet.createWallet(POOL, WALLET, TYPE, null, null).get();
	}

	@Test
	public void testCreateWalletWorksForPlugged() throws Exception {
		Wallet.createWallet(POOL, "pluggedWalletCreate", "inmem", null, null).get();
	}

	@Test
	public void testCreateWalletWorksForEmptyType() throws Exception {
		Wallet.createWallet(POOL, WALLET, null, null, null).get();
	}

	@Test
	public void testCreateWalletWorksForConfigJson() throws Exception {
		Wallet.createWallet(POOL, WALLET, null, "{\"freshness_time\":1000}", null).get();
	}

	@Test
	public void testCreateWalletWorksForCredentialsJson() throws Exception {
		Wallet.createWallet(POOL, WALLET, null, null, "{\"key\":\"testkey\"}").get();
	}

	@Test
	public void testCreateWalletWorksForUnknowType() throws Exception {
		thrown.expect(ExecutionException.class);
		thrown.expectCause(isA(UnknownWalletTypeException.class));

		Wallet.createWallet(POOL, WALLET, "unknow_type", null, null).get();
	}

	@Test
	public void testCreateWalletWorksForEmptyName() throws Exception {
		thrown.expect(IllegalArgumentException.class);

		Wallet.createWallet(POOL, "", TYPE, null, null).get();
	}

	@Test
	public void testCreateWalletWorksForDuplicateName() throws Exception {
		thrown.expect(ExecutionException.class);
		thrown.expectCause(isA(WalletExistsException.class));

		Wallet.createWallet(POOL, WALLET, TYPE, null, null).get();
		Wallet.createWallet(POOL, WALLET, TYPE, null, null).get();
	}
}