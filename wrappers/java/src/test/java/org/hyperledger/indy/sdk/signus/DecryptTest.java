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
	private byte[] encryptedMessage = {-105, 30, 89, 75, 76, 28, -59, -45, 105, -46, 20, 124, -85, -13, 109, 29, -88, -82, -8, -6, -50, -84, -53, -48, -49, 56, 124, 114, 82, 126, 74, 99, -72, -78, -117, 96, 60, 119, 50, -40, 121, 21, 57, -68, 89};
	private byte[] nonce = {-14, 102, -41, -57, 1, 4, 75, -46, -91, 87, 14, 41, -39, 48, 42, -126, -121, 84, -58, 59, -27, 51, -32, -23};

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
		byte[] decryptedMessage = Signus.decrypt(wallet, myDid, trusteeDid, encryptedMessage, nonce).get();
		assertTrue(Arrays.equals(MESSAGE, decryptedMessage));
	}

	@Test
	public void testDecryptWorksForOtherCoder() throws Exception {
		thrown.expect(ExecutionException.class);
		thrown.expectCause(isA(InvalidStructureException.class));

		String identityJson = String.format(IDENTITY_JSON_TEMPLATE, myDid, myVerkey);
		Signus.storeTheirDid(wallet, identityJson).get();

		SignusResults.EncryptResult encryptResult = Signus.encrypt(wallet, pool, myDid, myDid, MESSAGE).get();

		Signus.decrypt(wallet, myDid, trusteeDid, encryptResult.getEncryptedMessage(), encryptResult.getNonce()).get();
	}

	@Test
	public void testDecryptWorksForNonceNotCorrespondMessage() throws Exception {
		thrown.expect(ExecutionException.class);
		thrown.expectCause(isA(InvalidStructureException.class));

		byte[] nonce = {46, 33, -4, 67, 1, 44, 57, -46, -91, 87, 14, 41, -39, 48, 42, -126, -121, 84, -58, 59, -27, 51, -32, -23};

		Signus.decrypt(wallet, myDid, trusteeDid, encryptedMessage, nonce).get();
	}

	@Test
	public void testDecryptWorksForUnknownMyDid() throws Exception {
		thrown.expect(ExecutionException.class);
		thrown.expectCause(isA(WalletValueNotFoundException.class));

		Signus.decrypt(wallet, "unknowDid", trusteeDid, encryptedMessage, nonce).get();
	}
}
