package org.hyperledger.indy.sdk.wallet;

import org.hyperledger.indy.sdk.ErrorCode;
import org.hyperledger.indy.sdk.ErrorCodeMatcher;
import org.hyperledger.indy.sdk.utils.InitHelper;
import org.hyperledger.indy.sdk.utils.StorageUtils;
import org.hyperledger.indy.sdk.wallet.WalletResults.CreateWalletResult;
import static org.junit.Assert.assertNotNull;
import org.junit.Before;
import org.junit.Ignore;
import org.junit.Rule;
import org.junit.Test;
import org.junit.rules.ExpectedException;

import java.util.concurrent.ExecutionException;


public class DeleteWalletTest {

	@Rule
	public ExpectedException thrown = ExpectedException.none();

	@Before
	public void setUp() throws Exception {
		InitHelper.init();
	}

	@Test
	public void testDeleteWalletWorks() throws Exception {

		StorageUtils.cleanupStorage();

		CreateWalletResult result1 = Wallet.createWallet("default", "deleteWalletWorks", "default", null, null).get();
		assertNotNull(result1);

		WalletResults.DeleteWalletResult result4 = Wallet.deleteWallet("deleteWalletWorks", null).get();
		assertNotNull(result4);

		CreateWalletResult result3 = Wallet.createWallet("default", "deleteWalletWorks", "default", null, null).get();
		assertNotNull(result3);

		StorageUtils.cleanupStorage();
	}

	@Test
	public void testDeleteWalletWorksForClosed() throws Exception {

		StorageUtils.cleanupStorage();

		Wallet wallet;

		CreateWalletResult result1 = Wallet.createWallet("default", "deleteWalletWorksForClosed",
				null, null, null).get();
		assertNotNull(result1);

		WalletResults.OpenWalletResult result2 = Wallet.openWallet("deleteWalletWorksForClosed", null, null).get();
		assertNotNull(result2);
		wallet = result2.getWallet();

		WalletResults.CloseWalletResult result3 = wallet.closeWallet().get();
		assertNotNull(result3);

		WalletResults.DeleteWalletResult result4 = Wallet.deleteWallet("deleteWalletWorksForClosed", null).get();
		assertNotNull(result4);

		CreateWalletResult result5 = Wallet.createWallet("default", "deleteWalletWorksForClosed", null, null, null).get();
		assertNotNull(result5);

		StorageUtils.cleanupStorage();
	}

	@Test
	@Ignore//TODO THERE IS BUG IN INDY
	public void testDeleteWalletWorksForOpened() throws Exception {

		StorageUtils.cleanupStorage();

		thrown.expect(ExecutionException.class);
		thrown.expectCause(new ErrorCodeMatcher(ErrorCode.CommonIOError));

		CreateWalletResult result1 = Wallet.createWallet("default", "deleteWalletWorksForOpened", null, null, null).get();
		assertNotNull(result1);

		WalletResults.OpenWalletResult result2 = Wallet.openWallet("deleteWalletWorksForOpened", null, null).get();
		assertNotNull(result2);

		Wallet.deleteWallet("deleteWalletWorksForOpened", null).get();

		StorageUtils.cleanupStorage();
	}

	@Test
	public void testDeleteWalletWorksForTwice() throws Exception {

		StorageUtils.cleanupStorage();

		thrown.expect(ExecutionException.class);
		thrown.expectCause(new ErrorCodeMatcher(ErrorCode.CommonIOError));

		Wallet wallet;

		CreateWalletResult result1 = Wallet.createWallet("default", "deleteWalletWorksForTwice",
				null, null, null).get();
		assertNotNull(result1);

		WalletResults.OpenWalletResult result2 = Wallet.openWallet("deleteWalletWorksForTwice", null, null).get();
		assertNotNull(result2);
		wallet = result2.getWallet();

		WalletResults.CloseWalletResult result3 = wallet.closeWallet().get();
		assertNotNull(result3);

		WalletResults.DeleteWalletResult result4 = Wallet.deleteWallet("deleteWalletWorksForTwice", null).get();
		assertNotNull(result4);

		Wallet.deleteWallet("deleteWalletWorksForTwice", null).get();

		StorageUtils.cleanupStorage();
	}
}
