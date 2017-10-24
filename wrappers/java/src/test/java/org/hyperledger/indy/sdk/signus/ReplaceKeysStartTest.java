package org.hyperledger.indy.sdk.signus;

import org.bitcoinj.core.Base58;
import org.hyperledger.indy.sdk.IndyIntegrationTestWithSingleWallet;
import org.hyperledger.indy.sdk.signus.SignusResults.CreateAndStoreMyDidResult;
import org.hyperledger.indy.sdk.signus.SignusResults.ReplaceKeysStartResult;
import org.hyperledger.indy.sdk.wallet.WalletValueNotFoundException;

import static org.hamcrest.CoreMatchers.isA;
import static org.junit.Assert.assertEquals;
import static org.junit.Assert.assertNotEquals;

import org.junit.Before;
import org.junit.Test;

import java.util.concurrent.ExecutionException;

public class ReplaceKeysStartTest extends IndyIntegrationTestWithSingleWallet {

	private String did;
	private String verkey;

	@Before
	public void before() throws Exception {
		CreateAndStoreMyDidResult result = Signus.createAndStoreMyDid(this.wallet, "{}").get();
		did = result.getDid();
		verkey = result.getVerkey();
	}

	@Test
	public void testReplaceKeysStartWorksForEmptyJson() throws Exception {
		ReplaceKeysStartResult result = Signus.replaceKeysStart(wallet, did, "{}").get();
		assertEquals(32, Base58.decode(result.getVerkey()).length);
	}

	@Test
	public void testReplaceKeysStartWorksForNotExistsDid() throws Exception {
		thrown.expect(ExecutionException.class);
		thrown.expectCause(isA(WalletValueNotFoundException.class));

		Signus.replaceKeysStart(this.wallet, DID1, "{}").get();
	}

	@Test
	public void testReplaceKeysStartWorksForSeed() throws Exception {
		ReplaceKeysStartResult result = Signus.replaceKeysStart(this.wallet, this.did, "{\"seed\":\"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa\"}").get();
		String verkey = result.getVerkey();

		assertEquals("CnEDk9HrMnmiHXEV1WFgbVCRteYnPqsJwrTdcZaNhFVW", verkey);
		assertNotEquals(this.verkey, verkey);
	}
}
