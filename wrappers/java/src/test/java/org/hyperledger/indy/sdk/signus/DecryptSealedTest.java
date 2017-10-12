package org.hyperledger.indy.sdk.signus;

import org.hyperledger.indy.sdk.ErrorCode;
import org.hyperledger.indy.sdk.ErrorCodeMatcher;
import org.hyperledger.indy.sdk.IndyIntegrationTestWithPoolAndSingleWallet;
import org.hyperledger.indy.sdk.signus.SignusResults.CreateAndStoreMyDidResult;
import org.junit.Before;
import org.junit.Test;

import java.util.Arrays;
import java.util.concurrent.ExecutionException;

import static org.junit.Assert.assertTrue;

public class DecryptSealedTest extends IndyIntegrationTestWithPoolAndSingleWallet {

	private String trusteeDid;
	private String trusteeVerkey;
	private String myDid;
	private String myVerkey;

	@Before
	public void before() throws Exception {
		CreateAndStoreMyDidResult trusteeNym = Signus.createAndStoreMyDid(wallet, TRUSTEE_IDENTITY_JSON).get();
		trusteeDid = trusteeNym.getDid();
		trusteeVerkey = trusteeNym.getVerkey();

		CreateAndStoreMyDidResult myNym = Signus.createAndStoreMyDid(wallet, MY1_IDENTITY_JSON).get();
		myDid = myNym.getDid();
		myVerkey = myNym.getVerkey();

		String identityJson = String.format(IDENTITY_JSON_TEMPLATE, trusteeDid, trusteeVerkey);
		Signus.storeTheirDid(wallet, identityJson).get();
	}

	@Test
	public void testDecryptSealedWorks() throws Exception {
		String identityJson = String.format(IDENTITY_JSON_TEMPLATE, trusteeDid, trusteeVerkey);
		Signus.storeTheirDid(wallet, identityJson).get();

		byte[] encryptedMessage = Signus.encryptSealed(wallet, pool, trusteeDid, MESSAGE).get();
		byte[] decryptedMessage = Signus.decryptSealed(wallet, trusteeDid, encryptedMessage).get();
		assertTrue(Arrays.equals(MESSAGE, decryptedMessage));
	}

	@Test
	public void testSealedDecryptSealedWorksForOtherCoder() throws Exception {
		thrown.expect(ExecutionException.class);
		thrown.expectCause(new ErrorCodeMatcher(ErrorCode.CommonInvalidStructure));

		String identityJson = String.format(IDENTITY_JSON_TEMPLATE, myDid, myVerkey);
		Signus.storeTheirDid(wallet, identityJson).get();

		byte[] encryptResult = Signus.encryptSealed(wallet, pool, myDid, MESSAGE).get();

		Signus.decryptSealed(wallet, trusteeDid, encryptResult).get();
	}

	@Test
	public void testDecryptSealedWorksForUnknownMyDid() throws Exception {
		thrown.expect(ExecutionException.class);
		thrown.expectCause(new ErrorCodeMatcher(ErrorCode.WalletNotFoundError));

		byte[] encryptedMessage = {- 105, 30, 89, 75, 76, 28, - 59, - 45, 105, - 46, 20};
		Signus.decryptSealed(wallet, "unknowDid", encryptedMessage).get();
	}
}
