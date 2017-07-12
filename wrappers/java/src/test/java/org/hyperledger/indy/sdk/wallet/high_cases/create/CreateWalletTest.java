package org.hyperledger.indy.sdk.wallet.high_cases.create;

import java.util.concurrent.ExecutionException;

import org.hyperledger.indy.sdk.helpres.InitHelper;
import org.hyperledger.indy.sdk.wallet.Wallet;
import org.hyperledger.indy.sdk.wallet.WalletResults.CreateWalletResult;
import org.hyperledger.indy.sdk.helpres.StorageHelper;
import org.junit.Assert;
import org.junit.Before;
import org.junit.Rule;
import org.junit.Test;
import org.junit.rules.ExpectedException;


public class CreateWalletTest {

	@Rule
	public ExpectedException thrown = ExpectedException.none();

	@Before
	public void setUp() throws Exception {
		InitHelper.init();
	}

	@Test
	public void testCreateWalletWorks() throws Exception {

		StorageHelper.cleanupStorage();

		CreateWalletResult result1 = Wallet.createWallet("default", "mywallet", "default", null, null).get();
		Assert.assertNotNull(result1);

		StorageHelper.cleanupStorage();
	}

	@Test
	public void testCreateWalletWorksForEmptyType() throws Exception {

		StorageHelper.cleanupStorage();

		CreateWalletResult result1 = Wallet.createWallet("default", "mywallet", null, null, null).get();
		Assert.assertNotNull(result1);

		StorageHelper.cleanupStorage();
	}

	@Test
	public void testCreateWalletWorksForConfigJson() throws Exception {

		StorageHelper.cleanupStorage();

		CreateWalletResult result1 = Wallet.createWallet("default", "mywallet", null,
				"{\"freshness_time\":1000}", null).get();
		Assert.assertNotNull(result1);

		StorageHelper.cleanupStorage();
	}

	@Test
	public void testCreateWalletWorksForUnknowType() throws Exception {

		StorageHelper.cleanupStorage();

		thrown.expect(ExecutionException.class);
		thrown.expectMessage("WalletUnknownTypeError: 201");

		Wallet.createWallet("default", "mywallet", "unknow_type", null, null).get();

		StorageHelper.cleanupStorage();
	}
}