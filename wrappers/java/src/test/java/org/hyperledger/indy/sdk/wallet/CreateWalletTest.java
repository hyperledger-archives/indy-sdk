package org.hyperledger.indy.sdk.wallet;

import java.util.concurrent.ExecutionException;

import org.hyperledger.indy.sdk.ErrorCode;
import org.hyperledger.indy.sdk.ErrorCodeMatcher;
import org.hyperledger.indy.sdk.IndyIntegrationTest;
import org.hyperledger.indy.sdk.utils.InitHelper;
import org.hyperledger.indy.sdk.wallet.WalletResults.CreateWalletResult;
import org.hyperledger.indy.sdk.utils.StorageUtils;
import static org.junit.Assert.assertNotNull;
import org.junit.Before;
import org.junit.Rule;
import org.junit.Test;
import org.junit.rules.ExpectedException;


public class CreateWalletTest extends IndyIntegrationTest {

	@Test
	public void testCreateWalletWorks() throws Exception {

		StorageUtils.cleanupStorage();

		CreateWalletResult result1 = Wallet.createWallet("default", "createWalletWorks", "default", null, null).get();
		assertNotNull(result1);

		StorageUtils.cleanupStorage();
	}

	@Test
	public void testCreateWalletWorksForEmptyType() throws Exception {

		StorageUtils.cleanupStorage();

		CreateWalletResult result1 = Wallet.createWallet("default", "createWalletWorks", null, null, null).get();
		assertNotNull(result1);

		StorageUtils.cleanupStorage();
	}

	@Test
	public void testCreateWalletWorksForConfigJson() throws Exception {

		StorageUtils.cleanupStorage();

		CreateWalletResult result1 = Wallet.createWallet("default", "createWalletWorks", null,
				"{\"freshness_time\":1000}", null).get();
		assertNotNull(result1);

		StorageUtils.cleanupStorage();
	}

	@Test
	public void testCreateWalletWorksForUnknowType() throws Exception {

		StorageUtils.cleanupStorage();

		thrown.expect(ExecutionException.class);
		thrown.expectCause(new ErrorCodeMatcher(ErrorCode.WalletUnknownTypeError));

		Wallet.createWallet("default", "createWalletWorks", "unknow_type", null, null).get();

		StorageUtils.cleanupStorage();
	}

	@Test
	public void testCreateWalletWorksForEmptyName() throws Exception {

		StorageUtils.cleanupStorage();

		thrown.expect(new ErrorCodeMatcher(ErrorCode.CommonInvalidParam2));

		Wallet.createWallet("", "createWalletWorks", "default", null, null).get();

		StorageUtils.cleanupStorage();
	}

	@Test
	public void testCreateWalletWorksForDuplicateName() throws Exception {

		StorageUtils.cleanupStorage();

		thrown.expect(ExecutionException.class);
		thrown.expectCause(new ErrorCodeMatcher(ErrorCode.WalletAlreadyExistsError));

		CreateWalletResult result1 = Wallet.createWallet("default", "createWalletWorks", "default", null, null).get();
		assertNotNull(result1);
		Wallet.createWallet("default", "createWalletWorks", "default", null, null).get();

		StorageUtils.cleanupStorage();
	}
}