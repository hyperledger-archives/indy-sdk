package org.hyperledger.indy.sdk.wallet;

import org.hyperledger.indy.sdk.ErrorCode;
import org.hyperledger.indy.sdk.ErrorCodeMatcher;
import org.hyperledger.indy.sdk.IndyIntegrationTest;
import static org.junit.Assert.assertNotNull;
import org.junit.Ignore;
import org.junit.Test;

import java.util.concurrent.ExecutionException;


public class DeleteWalletTest extends IndyIntegrationTest {

	@Test
	public void testDeleteWalletWorks() throws Exception {

		Wallet.createWallet("default", "deleteWalletWorks", "default", null, null).get();
		Wallet.deleteWallet("deleteWalletWorks", null).get();
		Wallet.createWallet("default", "deleteWalletWorks", "default", null, null).get();
	}

	@Test
	public void testDeleteWalletWorksForClosed() throws Exception {

		Wallet.createWallet("default", "deleteWalletWorksForClosed", null, null, null).get();

		Wallet wallet = Wallet.openWallet("deleteWalletWorksForClosed", null, null).get();
		assertNotNull(wallet);

		wallet.closeWallet().get();
		Wallet.deleteWallet("deleteWalletWorksForClosed", null).get();
		Wallet.createWallet("default", "deleteWalletWorksForClosed", null, null, null).get();
	}

	@Test
	@Ignore//TODO THERE IS BUG IN INDY
	public void testDeleteWalletWorksForOpened() throws Exception {

		thrown.expect(ExecutionException.class);
		thrown.expectCause(new ErrorCodeMatcher(ErrorCode.CommonIOError));

		Wallet.createWallet("default", "deleteWalletWorksForOpened", null, null, null).get();
		Wallet.openWallet("deleteWalletWorksForOpened", null, null).get();
		Wallet.deleteWallet("deleteWalletWorksForOpened", null).get();
	}

	@Test
	public void testDeleteWalletWorksForTwice() throws Exception {

		thrown.expect(ExecutionException.class);
		thrown.expectCause(new ErrorCodeMatcher(ErrorCode.CommonIOError));

		Wallet.createWallet("default", "deleteWalletWorksForTwice", null, null, null).get();

		Wallet wallet = Wallet.openWallet("deleteWalletWorksForTwice", null, null).get();
		assertNotNull(wallet);

		wallet.closeWallet().get();

		Wallet.deleteWallet("deleteWalletWorksForTwice", null).get();
		Wallet.deleteWallet("deleteWalletWorksForTwice", null).get();
	}

	@Test
	public void testDeleteWalletWorksForNotCreated() throws Exception {

		thrown.expect(ExecutionException.class);
		thrown.expectCause(new ErrorCodeMatcher(ErrorCode.CommonIOError));

		Wallet.deleteWallet("deleteWalletWorksForTwice", null).get();
	}
}
