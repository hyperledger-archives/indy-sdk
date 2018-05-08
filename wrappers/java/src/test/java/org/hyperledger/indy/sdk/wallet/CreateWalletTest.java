package org.hyperledger.indy.sdk.wallet;

import static org.hamcrest.CoreMatchers.isA;

import java.util.concurrent.ExecutionException;

import org.hyperledger.indy.sdk.IndyIntegrationTest;
import org.junit.Ignore;
import org.junit.Test;


public class CreateWalletTest extends IndyIntegrationTest {

	@Test
	public void testCreateWalletWorks() throws Exception {
		Wallet.createWallet(POOL, WALLET, TYPE, null, CREDENTIALS).get();
	}

	@Test
	@Ignore
	public void testCreateWalletWorksForPlugged() throws Exception {
		Wallet.createWallet(POOL, "pluggedWalletCreate", "inmem", null, CREDENTIALS).get();
	}

	@Test
	public void testCreateWalletWorksForEmptyType() throws Exception {
		Wallet.createWallet(POOL, WALLET, null, null, CREDENTIALS).get();
	}

	@Test
	public void testCreateWalletWorksForConfigJson() throws Exception {
		Wallet.createWallet(POOL, WALLET, null, "{\"freshness_time\":1000}", CREDENTIALS).get();
	}

	@Test
	public void testCreateWalletWorksForUnknowType() throws Exception {
		thrown.expect(ExecutionException.class);
		thrown.expectCause(isA(UnknownWalletTypeException.class));

		Wallet.createWallet(POOL, WALLET, "unknow_type", null, CREDENTIALS).get();
	}

	@Test
	public void testCreateWalletWorksForEmptyName() throws Exception {
		thrown.expect(IllegalArgumentException.class);

		Wallet.createWallet(POOL, "", TYPE, null, CREDENTIALS).get();
	}

	@Test
	public void testCreateWalletWorksForDuplicateName() throws Exception {
		thrown.expect(ExecutionException.class);
		thrown.expectCause(isA(WalletExistsException.class));

		Wallet.createWallet(POOL, WALLET, TYPE, null, CREDENTIALS).get();
		Wallet.createWallet(POOL, WALLET, TYPE, null, CREDENTIALS).get();
	}
}