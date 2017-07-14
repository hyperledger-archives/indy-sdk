package org.hyperledger.indy.sdk.signus;

import org.bitcoinj.core.Base58;
import org.hyperledger.indy.sdk.ErrorCode;
import org.hyperledger.indy.sdk.ErrorCodeMatcher;
import org.hyperledger.indy.sdk.IndyIntegrationTest;
import org.hyperledger.indy.sdk.signus.SignusResults.CreateAndStoreMyDidResult;
import org.hyperledger.indy.sdk.wallet.Wallet;

import static org.junit.Assert.assertNotNull;
import static org.junit.Assert.assertEquals;
import static org.junit.Assert.assertNotEquals;

import org.junit.After;
import org.junit.Before;
import org.junit.Test;

import java.util.concurrent.ExecutionException;

public class ReplaceKeysTest extends IndyIntegrationTest {

	private Wallet wallet;
	private String did;
	private String verkey;

	@Before
	public void createWalletWithDid() throws Exception {
		Wallet.createWallet("default", "signusWallet", "default", null, null).get();
		wallet = Wallet.openWallet("signusWallet", null, null).get();

		SignusJSONParameters.CreateAndStoreMyDidJSONParameter didJson =
				new SignusJSONParameters.CreateAndStoreMyDidJSONParameter(null, null, null, null);

		CreateAndStoreMyDidResult result = Signus.createAndStoreMyDid(this.wallet, didJson.toJson()).get();

		did = result.getDid();
		verkey = result.getVerkey();
	}

	@After
	public void deleteWallet() throws Exception {
		wallet.closeWallet().get();
		Wallet.deleteWallet("signusWallet", null).get();
	}

	@Test
	public void testReplaceKeysWorksForEmptyJson() throws Exception {
		SignusResults.ReplaceKeysResult result = Signus.replaceKeys(wallet, did, "{}").get();
		assertNotNull(result);

		assertEquals(32, Base58.decode(result.getVerkey()).length);
	}

	@Test
	public void testReplaceKeysWorksForInvalidDid() throws Exception {
		thrown.expect(ExecutionException.class);
		thrown.expectCause(new ErrorCodeMatcher(ErrorCode.CommonInvalidStructure));

		Signus.replaceKeys(this.wallet, "invalid_base58_string", "{}").get();
	}

	@Test
	public void testReplaceKeysWorksForNotExistsDid() throws Exception {
		SignusResults.ReplaceKeysResult result = Signus.replaceKeys(this.wallet, "8wZcEriaNLNKtteJvx7f8i", "{}").get();
		assertNotNull(result);
	}

	@Test
	public void testReplaceKeysWorksForSeed() throws Exception {
		SignusResults.ReplaceKeysResult result = Signus.replaceKeys(this.wallet, this.did, "{\"seed\":\"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa\"}").get();
		assertNotNull(result);
		String verkey = result.getVerkey();

		assertEquals("CnEDk9HrMnmiHXEV1WFgbVCRteYnPqsJwrTdcZaNhFVW", verkey);
		assertNotEquals(this.verkey, verkey);
	}
}
