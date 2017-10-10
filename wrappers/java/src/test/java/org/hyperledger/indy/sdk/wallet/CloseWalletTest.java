package org.hyperledger.indy.sdk.wallet;

import org.hyperledger.indy.sdk.ErrorCode;
import org.hyperledger.indy.sdk.ErrorCodeMatcher;
import org.hyperledger.indy.sdk.IndyIntegrationTest;

import org.junit.Test;

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
		thrown.expectCause(new ErrorCodeMatcher(ErrorCode.WalletInvalidHandle));

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
}