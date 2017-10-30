package org.hyperledger.indy.sdk.crypto;

import org.hyperledger.indy.sdk.IndyIntegrationTestWithSingleWallet;
import org.hyperledger.indy.sdk.wallet.WalletValueNotFoundException;
import org.junit.Test;

import java.util.concurrent.ExecutionException;

import static org.hamcrest.CoreMatchers.isA;

public class CryptoBoxTest extends IndyIntegrationTestWithSingleWallet {

	@Test
	public void testCryptoBoxWorks() throws Exception {
		String myVk = Crypto.createKey(wallet, MY1_IDENTITY_KEY_JSON).get();
		Crypto.cryptoBox(wallet, myVk, VERKEY_MY2, MESSAGE).get();
	}

	@Test
	public void testCryptoBoxWorksForUnknownCoder() throws Exception {
		thrown.expect(ExecutionException.class);
		thrown.expectCause(isA(WalletValueNotFoundException.class));

		Crypto.cryptoBox(wallet, VERKEY_MY1, VERKEY_MY2, MESSAGE).get();
	}
}
