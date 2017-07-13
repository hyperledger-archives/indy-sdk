package org.hyperledger.indy.sdk.wallet;

import org.hyperledger.indy.sdk.ErrorCode;
import org.hyperledger.indy.sdk.ErrorCodeMatcher;
import org.hyperledger.indy.sdk.IndyIntegrationTest;
import org.hyperledger.indy.sdk.utils.StorageUtils;
import org.hyperledger.indy.sdk.utils.InitHelper;

import static org.junit.Assert.assertNotNull;

import org.junit.Before;
import org.junit.Rule;
import org.junit.Test;
import org.junit.rules.ExpectedException;

import java.util.concurrent.ExecutionException;


public class OpenWalletTest extends IndyIntegrationTest {

	@Test
	public void testOpenWalletWorks() throws Exception {

		StorageUtils.cleanupStorage();

		Wallet.createWallet("default", "openWalletWorks", "default", null, null).get();

		Wallet wallet = Wallet.openWallet("openWalletWorks", null, null).get();
		assertNotNull(wallet);

		StorageUtils.cleanupStorage();
	}

	@Test
	public void testOpenWalletWorksForConfig() throws Exception {

		StorageUtils.cleanupStorage();

		Wallet.createWallet("default", "openWalletWorksForConfig", "default", null, null).get();

		Wallet wallet = Wallet.openWallet("openWalletWorksForConfig", "{\"freshness_time\":1000}", null).get();
		assertNotNull(wallet);

		StorageUtils.cleanupStorage();
	}

	@Test
	public void testOpenWalletWorksForNotCreatedWallet() throws Exception {

		StorageUtils.cleanupStorage();

		thrown.expect(ExecutionException.class);
		thrown.expectCause(new ErrorCodeMatcher(ErrorCode.CommonIOError));

		Wallet.openWallet("openWalletWorksForNotCreatedWallet", null, null).get();

		StorageUtils.cleanupStorage();
	}

	@Test
	public void testOpenWalletWorksForTwice() throws Exception {

		StorageUtils.cleanupStorage();

		thrown.expect(ExecutionException.class);
		thrown.expectCause(new ErrorCodeMatcher(ErrorCode.WalletAlreadyOpenedError));

		Wallet.createWallet("default", "openWalletWorksForTwice", "default", null, null).get();

		Wallet.openWallet("openWalletWorksForTwice", null, null).get();
		Wallet.openWallet("openWalletWorksForTwice", null, null).get();

		StorageUtils.cleanupStorage();
	}
}
