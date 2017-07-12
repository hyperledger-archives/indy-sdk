package org.hyperledger.indy.sdk;

import org.hyperledger.indy.sdk.helpres.InitHelper;
import org.hyperledger.indy.sdk.wallet.Wallet;
import org.hyperledger.indy.sdk.wallet.WalletResults.CloseWalletResult;
import org.hyperledger.indy.sdk.wallet.WalletResults.CreateWalletResult;
import org.hyperledger.indy.sdk.wallet.WalletResults.DeleteWalletResult;
import org.hyperledger.indy.sdk.wallet.WalletResults.OpenWalletResult;
import org.hyperledger.indy.sdk.helpres.StorageHelper;
import org.junit.Assert;

import junit.framework.TestCase;

public class WalletTest extends TestCase {

	@Override
	protected void setUp() throws Exception {
		InitHelper.init();
	}

	public void testWallet() throws Exception {

		StorageHelper.cleanupStorage();

		Wallet wallet;

		CreateWalletResult result1 = Wallet.createWallet("default", "mywallet", null, null, null).get();
		Assert.assertNotNull(result1);

		OpenWalletResult result2 = Wallet.openWallet("mywallet", null, null).get();
		Assert.assertNotNull(result2);
		wallet = result2.getWallet();

		CloseWalletResult result3 = wallet.closeWallet().get();
		Assert.assertNotNull(result3);

		DeleteWalletResult result4 = Wallet.deleteWallet("mywallet", null).get();
		Assert.assertNotNull(result4);

		StorageHelper.cleanupStorage();
	}
}
