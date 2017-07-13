package org.hyperledger.indy.sdk.wallet;

import org.hyperledger.indy.sdk.ErrorCode;
import org.hyperledger.indy.sdk.ErrorCodeMatcher;
import org.hyperledger.indy.sdk.IndyIntegrationTest;
import org.hyperledger.indy.sdk.wallet.WalletResults.CreateWalletResult;
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

		CreateWalletResult result1 = Wallet.createWallet("default", "openWalletWorks", "default", null, null).get();
		assertNotNull(result1);

		WalletResults.OpenWalletResult result2 = Wallet.openWallet("openWalletWorks", null, null).get();
		assertNotNull(result2);
		assertNotNull(result2.getWallet());

		StorageUtils.cleanupStorage();
	}

	@Test
	public void testOpenWalletWorksForConfig() throws Exception {

		StorageUtils.cleanupStorage();

		CreateWalletResult result1 = Wallet.createWallet("default", "openWalletWorksForConfig", "default", null, null).get();
		assertNotNull(result1);

		WalletResults.OpenWalletResult result2 = Wallet.openWallet("openWalletWorksForConfig", "{\"freshness_time\":1000}", null).get();
		assertNotNull(result2);
		assertNotNull(result2.getWallet());

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

		CreateWalletResult result1 = Wallet.createWallet("default", "openWalletWorksForTwice", "default", null, null).get();
		assertNotNull(result1);

		Wallet.openWallet("openWalletWorksForTwice", null, null).get();
		Wallet.openWallet("openWalletWorksForTwice", null, null).get();

		StorageUtils.cleanupStorage();
	}
}
