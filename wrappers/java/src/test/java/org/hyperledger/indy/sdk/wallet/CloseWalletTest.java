package org.hyperledger.indy.sdk.wallet;

import org.hyperledger.indy.sdk.ErrorCode;
import org.hyperledger.indy.sdk.ErrorCodeMatcher;
import org.hyperledger.indy.sdk.utils.InitHelper;
import org.hyperledger.indy.sdk.utils.StorageUtils;
import org.hyperledger.indy.sdk.wallet.WalletResults.CreateWalletResult;
import static org.junit.Assert.assertNotNull;
import org.junit.Before;
import org.junit.Rule;
import org.junit.Test;
import org.junit.rules.ExpectedException;

import java.util.concurrent.ExecutionException;


public class CloseWalletTest {

	@Rule
	public ExpectedException thrown = ExpectedException.none();

	@Before
	public void setUp() throws Exception {
		InitHelper.init();
	}

	@Test
	public void testCloseWalletWorks() throws Exception {

		StorageUtils.cleanupStorage();

		Wallet wallet;

		CreateWalletResult result1 = Wallet.createWallet("default", "closeWalletWorks", "default", null, null).get();
		assertNotNull(result1);

		WalletResults.OpenWalletResult result2 = Wallet.openWallet("closeWalletWorks", null, null).get();
		assertNotNull(result2);
		wallet = result2.getWallet();

		WalletResults.CloseWalletResult result3 = wallet.closeWallet().get();
		assertNotNull(result3);

		StorageUtils.cleanupStorage();
	}

	@Test
	public void testCloseWalletWorksForTwice() throws Exception {

		StorageUtils.cleanupStorage();

		thrown.expect(ExecutionException.class);
		thrown.expectCause(new ErrorCodeMatcher(ErrorCode.WalletInvalidHandle));

		Wallet wallet;

		CreateWalletResult result1 = Wallet.createWallet("default", "closeWalletWorksForTwice", "default", null, null).get();
		assertNotNull(result1);

		WalletResults.OpenWalletResult result2 = Wallet.openWallet("closeWalletWorksForTwice", null, null).get();
		assertNotNull(result2);
		wallet = result2.getWallet();

		WalletResults.CloseWalletResult result3 = wallet.closeWallet().get();
		assertNotNull(result3);

		wallet.closeWallet().get();

		StorageUtils.cleanupStorage();
	}
}
