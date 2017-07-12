package org.hyperledger.indy.sdk.wallet.high_cases.create;

import org.hyperledger.indy.sdk.wallet.Wallet;
import org.hyperledger.indy.sdk.wallet.WalletResults;
import org.hyperledger.indy.sdk.wallet.WalletResults.CreateWalletResult;
import org.hyperledger.indy.sdk.helpres.StorageHelper;
import org.hyperledger.indy.sdk.helpres.InitHelper;
import org.junit.Assert;
import org.junit.Before;
import org.junit.Rule;
import org.junit.Test;
import org.junit.rules.ExpectedException;


public class OpenWalletTest {

	@Rule
	public ExpectedException thrown = ExpectedException.none();

	@Before
	public void setUp() throws Exception {
		InitHelper.init();
	}

	@Test
	public void testOpenWalletWorks() throws Exception {

		StorageHelper.cleanupStorage();

		CreateWalletResult result1 = Wallet.createWallet("default", "mywallet", "default", null, null).get();
		Assert.assertNotNull(result1);

		WalletResults.OpenWalletResult result2 = Wallet.openWallet("mywallet", null, null).get();
		Assert.assertNotNull(result2);
		Assert.assertNotNull(result2.getWallet());

		StorageHelper.cleanupStorage();
	}

	@Test
	public void testOpenWalletWorksForConfig() throws Exception {

		StorageHelper.cleanupStorage();

		CreateWalletResult result1 = Wallet.createWallet("default", "mywallet", "default", null, null).get();
		Assert.assertNotNull(result1);

		WalletResults.OpenWalletResult result2 = Wallet.openWallet("mywallet", "{\"freshness_time\":1000}", null).get();
		Assert.assertNotNull(result2);
		Assert.assertNotNull(result2.getWallet());

		StorageHelper.cleanupStorage();
	}
}
