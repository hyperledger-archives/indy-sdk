package org.hyperledger.indy.sdk.signus;

import org.hyperledger.indy.sdk.ErrorCode;
import org.hyperledger.indy.sdk.ErrorCodeMatcher;
import org.hyperledger.indy.sdk.IndyIntegrationTest;
import org.hyperledger.indy.sdk.signus.SignusResults.CreateAndStoreMyDidResult;
import org.hyperledger.indy.sdk.wallet.Wallet;
import org.junit.After;
import org.junit.Before;
import org.junit.Test;

import java.util.concurrent.ExecutionException;


public class ReplaceKeysApplyTest extends IndyIntegrationTest {

	private Wallet wallet;
	private String did;
	private String walletName = "signusWallet";

	@Before
	public void createWalletWithDid() throws Exception {
		Wallet.createWallet("default", walletName, "default", null, null).get();
		wallet = Wallet.openWallet(walletName, null, null).get();

		CreateAndStoreMyDidResult result = Signus.createAndStoreMyDid(this.wallet, "{}").get();

		did = result.getDid();
	}

	@After
	public void deleteWallet() throws Exception {
		wallet.closeWallet().get();
		Wallet.deleteWallet(walletName, null).get();
	}

	@Test
	public void testReplaceKeysApplyWorks() throws Exception {
		Signus.replaceKeysStart(wallet, did, "{}").get();
		Signus.replaceKeysApply(wallet, did).get();
	}

	@Test
	public void testReplaceKeysApplyWorksWithoutCallingReplaceStart() throws Exception {
		thrown.expect(ExecutionException.class);
		thrown.expectCause(new ErrorCodeMatcher(ErrorCode.WalletNotFoundError));

		Signus.replaceKeysApply(wallet, did).get();
	}

	@Test
	public void testReplaceKeysApplyWorksForNotFoundDid() throws Exception {
		thrown.expect(ExecutionException.class);
		thrown.expectCause(new ErrorCodeMatcher(ErrorCode.WalletNotFoundError));

		Signus.replaceKeysStart(wallet, did, "{}").get();
		Signus.replaceKeysApply(wallet, "unknowndid").get();
	}
}
