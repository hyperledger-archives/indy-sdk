package org.hyperledger.indy.sdk.wallet;

import java.util.concurrent.ExecutionException;

import org.hyperledger.indy.sdk.ErrorCode;
import org.hyperledger.indy.sdk.ErrorCodeMatcher;
import org.hyperledger.indy.sdk.IndyIntegrationTest;
import org.junit.Test;


public class CreateWalletTest extends IndyIntegrationTest {

	@Test
	public void testCreateWalletWorks() throws Exception {

		Wallet.createWallet("default", "createWalletWorks", "default", null, null).get();
	}

	@Test
	public void testCreateWalletWorksForPlugged() throws Exception {
		Wallet.createWallet("default", "createWalletWorks", "inmem", null, null).get();
	}

	@Test
	public void testCreateWalletWorksForEmptyType() throws Exception {

		Wallet.createWallet("default", "createWalletWorks", null, null, null).get();
	}

	@Test
	public void testCreateWalletWorksForConfigJson() throws Exception {

		Wallet.createWallet("default", "createWalletWorks", null, "{\"freshness_time\":1000}", null).get();
	}

	@Test
	public void testCreateWalletWorksForUnknowType() throws Exception {

		thrown.expect(ExecutionException.class);
		thrown.expectCause(new ErrorCodeMatcher(ErrorCode.WalletUnknownTypeError));

		Wallet.createWallet("default", "createWalletWorks", "unknow_type", null, null).get();
	}

	@Test
	public void testCreateWalletWorksForEmptyName() throws Exception {

		thrown.expect(new ErrorCodeMatcher(ErrorCode.CommonInvalidParam3));

		Wallet.createWallet("pool", "", "default", null, null).get();
	}

	@Test
	public void testCreateWalletWorksForDuplicateName() throws Exception {

		thrown.expect(ExecutionException.class);
		thrown.expectCause(new ErrorCodeMatcher(ErrorCode.WalletAlreadyExistsError));

		String poolName = "default";
		String walletName = "deleteWalletWorks";
		String type = "default";

		Wallet.createWallet(poolName, walletName, type, null, null).get();
		Wallet.createWallet(poolName, walletName, type, null, null).get();
	}
}