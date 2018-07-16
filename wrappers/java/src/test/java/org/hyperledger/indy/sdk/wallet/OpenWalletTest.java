package org.hyperledger.indy.sdk.wallet;

import org.hyperledger.indy.sdk.IndyIntegrationTest;

import static org.hamcrest.CoreMatchers.isA;
import static org.junit.Assert.assertNotNull;

import org.json.JSONObject;
import org.junit.Ignore;
import org.junit.Test;

import java.util.concurrent.ExecutionException;


public class OpenWalletTest extends IndyIntegrationTest {

	@Test
	public void testOpenWalletWorks() throws Exception {
		Wallet.createWallet(WALLET_CONFIG, WALLET_CREDENTIALS).get();

		Wallet wallet = Wallet.openWallet(WALLET_CONFIG, WALLET_CREDENTIALS).get();
		assertNotNull(wallet);

		wallet.closeWallet().get();
	}

	@Test
	@Ignore // TODO: FIXME error has been changed
	public void testOpenWalletWorksForInvalidCredentials() throws Exception {
		Wallet.createWallet(WALLET_CONFIG, WALLET_CREDENTIALS).get();

		thrown.expect(ExecutionException.class);
		thrown.expectCause(isA(WalletAccessFailedException.class));

		Wallet.openWallet(WALLET_CONFIG, "{\"key\": \"other_key\"}").get();
	}

	@Test
	public void testOpenWalletWorksForEncryptedWalletChangingCredentials() throws Exception {
		Wallet.createWallet(WALLET_CONFIG, WALLET_CREDENTIALS).get();

		Wallet wallet = Wallet.openWallet(WALLET_CONFIG, "{\"key\": \"key\", \"rekey\": \"other_key\"}").get();
		wallet.closeWallet().get();

		wallet = Wallet.openWallet(WALLET_CONFIG, "{\"key\": \"other_key\"}").get();
		wallet.closeWallet().get();
	}

	@Test
	@Ignore
	public void testOpenWalletWorksForPlugged() throws Exception {
		Wallet.createWallet(PLUGGED_WALLET_CONFIG, WALLET_CREDENTIALS).get();
		Wallet wallet = Wallet.openWallet(PLUGGED_WALLET_CONFIG, WALLET_CREDENTIALS).get();
		assertNotNull(wallet);

		wallet.closeWallet().get();
	}

	@Test
	public void testOpenWalletWorksForNotCreatedWallet() throws Exception {
		thrown.expect(ExecutionException.class);
		thrown.expectCause(isA(WalletNotFoundException.class));

		Wallet.openWallet(WALLET_CONFIG, WALLET_CREDENTIALS).get();
	}

	@Test
	@Ignore // TODO: FIXME checking has been lost
	public void testOpenWalletWorksForTwice() throws Exception {
		thrown.expect(ExecutionException.class);
		thrown.expectCause(isA(WalletAlreadyOpenedException.class));

		String config = new JSONObject()
				.put("id", "openWalletWorksForTwice")
				.toString();

		Wallet.createWallet(config, WALLET_CREDENTIALS).get();

		Wallet.openWallet(config, WALLET_CREDENTIALS).get();
		Wallet.openWallet(config, WALLET_CREDENTIALS).get();
	}
}
