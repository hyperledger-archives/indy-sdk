package org.hyperledger.indy.sdk.wallet.medium_cases.create;

import java.util.concurrent.ExecutionException;

import org.hyperledger.indy.sdk.IndyException;
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
	public void testCreateWalletWorksForEmptyName() throws Exception {

		StorageHelper.cleanupStorage();

		thrown.expect(IndyException.class);
		thrown.expectMessage("CommonInvalidParam2: 101");

		Wallet.createWallet("", "mywallet", "default", null, null).get();

		StorageHelper.cleanupStorage();
	}

	@Test
	public void testCreateWalletWorksForDuplicateName() throws Exception {

		StorageHelper.cleanupStorage();

		thrown.expect(ExecutionException.class);
		thrown.expectMessage("WalletAlreadyExistsError: 203");

		CreateWalletResult result1 = Wallet.createWallet("default", "mywallet", "default", null, null).get();
		Assert.assertNotNull(result1);
		Wallet.createWallet("default", "mywallet", "default", null, null).get();

		StorageHelper.cleanupStorage();
	}
}