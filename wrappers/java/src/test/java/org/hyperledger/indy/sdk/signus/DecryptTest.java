package org.hyperledger.indy.sdk.signus;

import org.hyperledger.indy.sdk.IndyIntegrationTestWithPoolAndSingleWallet;
import org.hyperledger.indy.sdk.InvalidStructureException;
import org.hyperledger.indy.sdk.signus.SignusResults.CreateAndStoreMyDidResult;
import org.hyperledger.indy.sdk.wallet.WalletValueNotFoundException;
import org.junit.Before;
import org.junit.Test;

import java.util.Arrays;
import java.util.concurrent.ExecutionException;

import static org.hamcrest.CoreMatchers.isA;
import static org.junit.Assert.assertTrue;

public class DecryptTest extends IndyIntegrationTestWithPoolAndSingleWallet {

	private String trusteeDid;
	private String myDid;
	private String myVerkey;

	@Before
	public void before() throws Exception {
		CreateAndStoreMyDidResult trusteeNym = Signus.createAndStoreMyDid(wallet, TRUSTEE_IDENTITY_JSON).get();
		trusteeDid = trusteeNym.getDid();
		String trusteeVerkey = trusteeNym.getVerkey();

		CreateAndStoreMyDidResult myNym = Signus.createAndStoreMyDid(wallet, MY1_IDENTITY_JSON).get();
		myDid = myNym.getDid();
		myVerkey = myNym.getVerkey();

		String identityJson = String.format(IDENTITY_JSON_TEMPLATE, trusteeDid, trusteeVerkey);
		Signus.storeTheirDid(wallet, identityJson).get();
	}

	@Test
	public void testDecryptWorks() throws Exception {
		byte[] decryptedMessage = Signus.decrypt(wallet, pool, myDid, trusteeDid, ENCRYPTED_MESSAGE, NONCE).get();
		assertTrue(Arrays.equals(MESSAGE, decryptedMessage));
	}

	@Test
	public void testDecryptWorksForOtherCoder() throws Exception {
		thrown.expect(ExecutionException.class);
		thrown.expectCause(isA(InvalidStructureException.class));

		String identityJson = String.format(IDENTITY_JSON_TEMPLATE, myDid, myVerkey);
		Signus.storeTheirDid(wallet, identityJson).get();

		SignusResults.EncryptResult encryptResult = Signus.encrypt(wallet, pool, myDid, myDid, MESSAGE).get();

		Signus.decrypt(wallet, pool, myDid, trusteeDid, encryptResult.getEncryptedMessage(), encryptResult.getNonce()).get();
	}

	@Test
	public void testDecryptWorksForNonceNotCorrespondMessage() throws Exception {
		thrown.expect(ExecutionException.class);
		thrown.expectCause(isA(InvalidStructureException.class));

		byte[] nonce = {46, 33, - 4, 67, 1, 44, 57, - 46, - 91, 87, 14, 41, - 39, 48, 42, - 126, - 121, 84, - 58, 59, - 27, 51, - 32, - 23};

		Signus.decrypt(wallet, pool, myDid, trusteeDid, ENCRYPTED_MESSAGE, nonce).get();
	}

	@Test
	public void testDecryptWorksForUnknownMyDid() throws Exception {
		thrown.expect(ExecutionException.class);
		thrown.expectCause(isA(WalletValueNotFoundException.class));

		Signus.decrypt(wallet, pool, DID, trusteeDid, ENCRYPTED_MESSAGE, NONCE).get();
	}
}
