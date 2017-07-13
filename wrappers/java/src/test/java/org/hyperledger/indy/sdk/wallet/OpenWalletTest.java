package org.hyperledger.indy.sdk.wallet;

import org.hyperledger.indy.sdk.ErrorCode;
import org.hyperledger.indy.sdk.ErrorCodeMatcher;
import org.hyperledger.indy.sdk.IndyIntegrationTest;
import static org.junit.Assert.assertNotNull;
import org.junit.Test;

import java.util.concurrent.ExecutionException;


public class OpenWalletTest extends IndyIntegrationTest {

	@Test
	public void testOpenWalletWorks() throws Exception {

		Wallet.createWallet("default", "openWalletWorks", "default", null, null).get();

		Wallet wallet = Wallet.openWallet("openWalletWorks", null, null).get();
		assertNotNull(wallet);

	}

	@Test
	public void testOpenWalletWorksForConfig() throws Exception {

		Wallet.createWallet("default", "openWalletWorksForConfig", "default", null, null).get();

		Wallet wallet = Wallet.openWallet("openWalletWorksForConfig", "{\"freshness_time\":1000}", null).get();
		assertNotNull(wallet);
	}

	@Test
	public void testOpenWalletWorksForNotCreatedWallet() throws Exception {

		thrown.expect(ExecutionException.class);
		thrown.expectCause(new ErrorCodeMatcher(ErrorCode.CommonIOError));

		Wallet.openWallet("openWalletWorksForNotCreatedWallet", null, null).get();
	}

	@Test
	public void testOpenWalletWorksForTwice() throws Exception {

		thrown.expect(ExecutionException.class);
		thrown.expectCause(new ErrorCodeMatcher(ErrorCode.WalletAlreadyOpenedError));

		Wallet.createWallet("default", "openWalletWorksForTwice", "default", null, null).get();

		Wallet.openWallet("openWalletWorksForTwice", null, null).get();
		Wallet.openWallet("openWalletWorksForTwice", null, null).get();
	}

	@Test
	public void testOpenWalletWorksForNotCreated() throws Exception {

		thrown.expect(ExecutionException.class);
		thrown.expectCause(new ErrorCodeMatcher(ErrorCode.CommonIOError));

		Wallet.openWallet("testOpenWalletWorksForNotCreated", null, null).get();
	}
}
