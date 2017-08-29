package org.hyperledger.indy.sdk.signus;

import org.hyperledger.indy.sdk.ErrorCode;
import org.hyperledger.indy.sdk.ErrorCodeMatcher;
import org.hyperledger.indy.sdk.IndyIntegrationTest;
import org.hyperledger.indy.sdk.signus.SignusResults.CreateAndStoreMyDidResult;
import org.hyperledger.indy.sdk.wallet.Wallet;

import static org.junit.Assert.assertEquals;
import static org.junit.Assert.assertNotNull;

import org.bitcoinj.core.Base58;
import org.junit.After;
import org.junit.Before;
import org.junit.Test;

import java.util.concurrent.ExecutionException;

public class CreateMyDidTest extends IndyIntegrationTest {

	private Wallet wallet;
	private String walletName = "signusWallet";

	@Before
	public void createWallet() throws Exception {

		Wallet.createWallet("default", walletName, "default", null, null).get();
		this.wallet = Wallet.openWallet(walletName, null, null).get();
	}

	@After
	public void deleteWallet() throws Exception {
		this.wallet.closeWallet().get();
		Wallet.deleteWallet(walletName, null).get();
	}

	private String seed = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";
	private String did = "8wZcEriaNLNKtteJvx7f8i";
	private String expectedVerkey = "CnEDk9HrMnmiHXEV1WFgbVCRteYnPqsJwrTdcZaNhFVW";
	private String existsCryptoType = "ed25519";
	private String expectedDid = "NcYxiDXkpYi6ov5FcYDi1e";


	@Test
	public void testCreateMyDidWorksForEmptyJson() throws Exception {

		SignusJSONParameters.CreateAndStoreMyDidJSONParameter didJson =
				new SignusJSONParameters.CreateAndStoreMyDidJSONParameter(null, null, null, null);

		CreateAndStoreMyDidResult result = Signus.createAndStoreMyDid(this.wallet, didJson.toJson()).get();
		assertNotNull(result);

		assertEquals(16, Base58.decode(result.getDid()).length);
		assertEquals(32, Base58.decode(result.getVerkey()).length);
	}

	@Test
	public void testCreateMyDidWorksForSeed() throws Exception {

		SignusJSONParameters.CreateAndStoreMyDidJSONParameter didJson =
				new SignusJSONParameters.CreateAndStoreMyDidJSONParameter(null, seed, null, null);

		CreateAndStoreMyDidResult result = Signus.createAndStoreMyDid(this.wallet, didJson.toJson()).get();
		assertNotNull(result);

		assertEquals(expectedDid, result.getDid());
		assertEquals(expectedVerkey, result.getVerkey());
	}

	@Test
	public void testCreateMyDidWorksAsCid() throws Exception {

		SignusJSONParameters.CreateAndStoreMyDidJSONParameter didJson =
				new SignusJSONParameters.CreateAndStoreMyDidJSONParameter(null, seed, null, true);

		CreateAndStoreMyDidResult result = Signus.createAndStoreMyDid(this.wallet, didJson.toJson()).get();
		assertNotNull(result);

		assertEquals(expectedVerkey, result.getDid());
		assertEquals(expectedVerkey, result.getVerkey());
	}

	@Test
	public void testCreateMyDidWorksForPassedDid() throws Exception {

		SignusJSONParameters.CreateAndStoreMyDidJSONParameter didJson =
				new SignusJSONParameters.CreateAndStoreMyDidJSONParameter(did, null, null, false);

		CreateAndStoreMyDidResult result = Signus.createAndStoreMyDid(this.wallet, didJson.toJson()).get();
		assertNotNull(result);

		assertEquals(did, result.getDid());
	}

	@Test
	public void testCreateMyDidWorksForCorrectCryptoType() throws Exception {

		SignusJSONParameters.CreateAndStoreMyDidJSONParameter didJson =
				new SignusJSONParameters.CreateAndStoreMyDidJSONParameter(null, seed, existsCryptoType, null);

		CreateAndStoreMyDidResult result = Signus.createAndStoreMyDid(this.wallet, didJson.toJson()).get();
		assertNotNull(result);

		assertEquals(expectedDid, result.getDid());
		assertEquals(expectedVerkey, result.getVerkey());
	}

	@Test
	public void testCreateMyDidWorksForInvalidSeed() throws Exception {

		thrown.expect(ExecutionException.class);
		thrown.expectCause(new ErrorCodeMatcher(ErrorCode.CommonInvalidStructure));

		SignusJSONParameters.CreateAndStoreMyDidJSONParameter didJson =
				new SignusJSONParameters.CreateAndStoreMyDidJSONParameter(null, "aaaaaaaaaaa", null, null);

		Signus.createAndStoreMyDid(this.wallet, didJson.toJson()).get();
	}

	@Test
	public void testCreateMyDidWorksForInvalidCryptoType() throws Exception {

		thrown.expect(ExecutionException.class);
		thrown.expectCause(new ErrorCodeMatcher(ErrorCode.SignusUnknownCryptoError));

		SignusJSONParameters.CreateAndStoreMyDidJSONParameter didJson =
				new SignusJSONParameters.CreateAndStoreMyDidJSONParameter(null, null, "crypto_type", null);

		Signus.createAndStoreMyDid(this.wallet, didJson.toJson()).get();
	}

	@Test
	public void testCreateMyDidWorksForAllParams() throws Exception {

		SignusJSONParameters.CreateAndStoreMyDidJSONParameter didJson =
				new SignusJSONParameters.CreateAndStoreMyDidJSONParameter(did, seed, existsCryptoType, true);

		CreateAndStoreMyDidResult result = Signus.createAndStoreMyDid(this.wallet, didJson.toJson()).get();
		assertNotNull(result);

		assertEquals(did, result.getDid());
		assertEquals(expectedVerkey, result.getVerkey());
	}
}
