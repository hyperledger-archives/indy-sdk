package org.hyperledger.indy.sdk.wallet.medium_cases.create;

import org.hyperledger.indy.sdk.wallet.Wallet;
import org.hyperledger.indy.sdk.wallet.WalletResults;
import org.hyperledger.indy.sdk.wallet.WalletResults.CreateWalletResult;
import org.hyperledger.indy.sdk.helpres.StorageHelper;
import org.hyperledger.indy.sdk.helpres.InitHelper;
import org.junit.*;
import org.junit.rules.ExpectedException;

import java.util.concurrent.ExecutionException;


public class OpenWalletTest {

	@Rule
	public ExpectedException thrown = ExpectedException.none();

	@Before
	public void setUp() throws Exception {
		InitHelper.init();
	}

	@Test
	public void testOpenWalletWorksForNotCreatedWallet() throws Exception {

		StorageHelper.cleanupStorage();

		thrown.expect(ExecutionException.class);
		thrown.expectMessage("CommonIOError: 114");

		Wallet.openWallet("mywallet", null, null).get();

		StorageHelper.cleanupStorage();
	}

	@Test
	public void testOpenWalletWorksForTwice() throws Exception {

		StorageHelper.cleanupStorage();

		thrown.expect(ExecutionException.class);
		thrown.expectMessage("CommonIOError: 114");//TODO FIX in IndySdk

		WalletResults.OpenWalletResult result2 = Wallet.openWallet("mywallet", null, null).get();
		Assert.assertNotNull(result2);
		Assert.assertNotNull(result2.getWallet());

		Wallet.openWallet("mywallet", null, null).get();

		StorageHelper.cleanupStorage();
	}
}