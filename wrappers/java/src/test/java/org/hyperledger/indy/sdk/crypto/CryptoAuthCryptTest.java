package org.hyperledger.indy.sdk.crypto;

import org.hyperledger.indy.sdk.IndyIntegrationTestWithSingleWallet;
import org.hyperledger.indy.sdk.InvalidStructureException;
import org.hyperledger.indy.sdk.did.Did;
import org.hyperledger.indy.sdk.did.DidJSONParameters;
import org.hyperledger.indy.sdk.did.DidResults;
import org.hyperledger.indy.sdk.wallet.WalletValueNotFoundException;
import org.junit.Test;

import java.util.concurrent.ExecutionException;

import static org.hamcrest.CoreMatchers.isA;
import static org.junit.Assert.*;


public class CryptoAuthCryptTest extends IndyIntegrationTestWithSingleWallet {

	@Test
	public void testAuthCryptWorksForCreatedKey() throws Exception {
		String paramJson = new CryptoJSONParameters.CreateKeyJSONParameter(MY1_SEED, null).toJson();
		String myVk = Crypto.createKey(wallet, paramJson).get();

		byte[] encryptedMsg = Crypto.authCrypt(wallet, myVk, VERKEY_MY2, MESSAGE).get();
		assertNotNull(encryptedMsg);
	}

	@Test
	public void testAuthCryptWorksForCreatedDid() throws Exception {
		String didJson = new DidJSONParameters.CreateAndStoreMyDidJSONParameter(null, MY1_SEED, null, false).toJson();
		DidResults.CreateAndStoreMyDidResult result = Did.createAndStoreMyDid(wallet, didJson).get();
		String myVk = result.getVerkey();

		byte[] encryptedMsg = Crypto.authCrypt(wallet, myVk, VERKEY_MY2, MESSAGE).get();
		assertNotNull(encryptedMsg);
	}

	@Test
	public void testAuthCryptWorksForCreatedDidAsCid() throws Exception {
		String didJson = new DidJSONParameters.CreateAndStoreMyDidJSONParameter(null, MY1_SEED, null, true).toJson();
		DidResults.CreateAndStoreMyDidResult result = Did.createAndStoreMyDid(wallet, didJson).get();
		String myVk = result.getVerkey();

		byte[] encryptedMsg = Crypto.authCrypt(wallet, myVk, VERKEY_MY2, MESSAGE).get();
		assertNotNull(encryptedMsg);
	}

	@Test
	public void testAuthCryptWorksForUnknownSenderVerkey() throws Exception {
		thrown.expect(ExecutionException.class);
		thrown.expectCause(isA(WalletValueNotFoundException.class));

		Crypto.authCrypt(wallet, VERKEY, VERKEY_MY2, MESSAGE).get();
	}

	@Test
	public void testAuthCryptWorksForInvalidTheirVk() throws Exception {
		String paramJson = new CryptoJSONParameters.CreateKeyJSONParameter(MY1_SEED, null).toJson();
		String myVk = Crypto.createKey(wallet, paramJson).get();

		thrown.expect(ExecutionException.class);
		thrown.expectCause(isA(InvalidStructureException.class));

		Crypto.authCrypt(wallet, myVk, INVALID_VERKEY, MESSAGE).get();
	}
}