package org.hyperledger.indy.sdk.signus;

import org.hyperledger.indy.sdk.IndyIntegrationTestWithSingleWallet;
import org.hyperledger.indy.sdk.wallet.WalletValueNotFoundException;
import org.junit.Before;
import org.junit.Test;

import static org.hamcrest.CoreMatchers.isA;

import java.util.concurrent.ExecutionException;


public class ReplaceKeysApplyTest extends IndyIntegrationTestWithSingleWallet {

	private String did;

	@Before
	public void before() throws Exception {
		did = Signus.createAndStoreMyDid(this.wallet, "{}").get().getDid();
	}

	@Test
	public void testReplaceKeysApplyWorks() throws Exception {
		Signus.replaceKeysStart(wallet, did, "{}").get();
		Signus.replaceKeysApply(wallet, did).get();
	}

	@Test
	public void testReplaceKeysApplyWorksWithoutCallingReplaceStart() throws Exception {
		thrown.expect(ExecutionException.class);
		thrown.expectCause(isA(WalletValueNotFoundException.class));

		Signus.replaceKeysApply(wallet, did).get();
	}

	@Test
	public void testReplaceKeysApplyWorksForNotFoundDid() throws Exception {
		thrown.expect(ExecutionException.class);
		thrown.expectCause(isA(WalletValueNotFoundException.class));

		Signus.replaceKeysStart(wallet, did, "{}").get();
		Signus.replaceKeysApply(wallet, DID1).get();
	}
}
