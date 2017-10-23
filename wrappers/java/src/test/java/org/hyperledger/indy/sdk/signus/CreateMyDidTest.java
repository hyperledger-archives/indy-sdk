package org.hyperledger.indy.sdk.signus;

import org.hyperledger.indy.sdk.IndyIntegrationTestWithSingleWallet;
import org.hyperledger.indy.sdk.InvalidStructureException;
import org.hyperledger.indy.sdk.signus.SignusResults.CreateAndStoreMyDidResult;

import static org.hamcrest.CoreMatchers.isA;
import static org.junit.Assert.assertEquals;

import org.bitcoinj.core.Base58;
import org.junit.Test;

import java.util.concurrent.ExecutionException;

public class CreateMyDidTest extends IndyIntegrationTestWithSingleWallet {

	private String existsCryptoType = "ed25519";
	private String expectedDid = "VsKV7grR1BUE29mG2Fm2kX";
	private String expectedVerkey = "GjZWsBLgZCR18aL468JAT7w9CZRiBnpxUPPgyQxh4voa";

	@Test
	public void testCreateMyDidWorksForEmptyJson() throws Exception {
		CreateAndStoreMyDidResult result = Signus.createAndStoreMyDid(wallet, "{}").get();

		assertEquals(16, Base58.decode(result.getDid()).length);
		assertEquals(32, Base58.decode(result.getVerkey()).length);
	}

	@Test
	public void testCreateMyDidWorksForSeed() throws Exception {
		CreateAndStoreMyDidResult result = Signus.createAndStoreMyDid(this.wallet, MY1_IDENTITY_JSON).get();

		assertEquals(expectedDid, result.getDid());
		assertEquals(expectedVerkey, result.getVerkey());
	}

	@Test
	public void testCreateMyDidWorksAsCid() throws Exception {
		String  didJson = new SignusJSONParameters.CreateAndStoreMyDidJSONParameter(null, MY1_SEED, null, true).toJson();

		CreateAndStoreMyDidResult result = Signus.createAndStoreMyDid(this.wallet, didJson).get();

		assertEquals(expectedVerkey, result.getDid());
		assertEquals(expectedVerkey, result.getVerkey());
	}

	@Test
	public void testCreateMyDidWorksForPassedDid() throws Exception {

		String didJson = new SignusJSONParameters.CreateAndStoreMyDidJSONParameter(DID1, null, null, false).toJson();

		CreateAndStoreMyDidResult result = Signus.createAndStoreMyDid(this.wallet, didJson).get();

		assertEquals(DID1, result.getDid());
	}

	@Test
	public void testCreateMyDidWorksForCorrectCryptoType() throws Exception {
		String didJson = new SignusJSONParameters.CreateAndStoreMyDidJSONParameter(null, MY1_SEED, existsCryptoType, null).toJson();

		CreateAndStoreMyDidResult result = Signus.createAndStoreMyDid(this.wallet, didJson).get();

		assertEquals(expectedDid, result.getDid());
		assertEquals(expectedVerkey, result.getVerkey());
	}

	@Test
	public void testCreateMyDidWorksForInvalidSeed() throws Exception {
		thrown.expect(ExecutionException.class);
		thrown.expectCause(isA(InvalidStructureException.class));

		String didJson = new SignusJSONParameters.CreateAndStoreMyDidJSONParameter(null, "aaaaaaaaaaa", null, null).toJson();

		Signus.createAndStoreMyDid(this.wallet, didJson).get();
	}

	@Test
	public void testCreateMyDidWorksForInvalidCryptoType() throws Exception {
		thrown.expect(ExecutionException.class);
		thrown.expectCause(isA(UnknownCryptoException.class));

		String didJson = new SignusJSONParameters.CreateAndStoreMyDidJSONParameter(null, MY1_SEED, "crypto_type", null).toJson();

		Signus.createAndStoreMyDid(this.wallet, didJson).get();
	}

	@Test
	public void testCreateMyDidWorksForAllParams() throws Exception {
		String didJson = new SignusJSONParameters.CreateAndStoreMyDidJSONParameter(DID1, MY1_SEED, existsCryptoType, true).toJson();

		CreateAndStoreMyDidResult result = Signus.createAndStoreMyDid(this.wallet, didJson).get();

		assertEquals(DID1, result.getDid());
		assertEquals(expectedVerkey, result.getVerkey());
	}
}
