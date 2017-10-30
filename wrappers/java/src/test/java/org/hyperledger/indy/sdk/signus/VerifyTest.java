package org.hyperledger.indy.sdk.signus;

import org.hyperledger.indy.sdk.IndyIntegrationTestWithPoolAndSingleWallet;
import org.hyperledger.indy.sdk.ledger.Ledger;
import org.hyperledger.indy.sdk.signus.SignusResults.CreateAndStoreMyDidResult;
import org.junit.Before;
import org.junit.Test;

import static junit.framework.TestCase.assertFalse;
import static org.junit.Assert.assertTrue;

public class VerifyTest extends IndyIntegrationTestWithPoolAndSingleWallet {

	private String trusteeDid;
	private String trusteeVerkey;
	private String myDid;
	private String myVerkey;

	@Before
	public void before() throws Exception {
		CreateAndStoreMyDidResult createDidResult = Signus.createAndStoreMyDid(wallet, TRUSTEE_IDENTITY_JSON).get();
		trusteeDid = createDidResult.getDid();
		trusteeVerkey = createDidResult.getVerkey();

		createDidResult = Signus.createAndStoreMyDid(wallet, MY1_IDENTITY_JSON).get();
		myDid = createDidResult.getDid();
		myVerkey = createDidResult.getVerkey();

		String nymRequest = Ledger.buildNymRequest(trusteeDid, myDid, myVerkey, null, null).get();
		Ledger.signAndSubmitRequest(pool, wallet, trusteeDid, nymRequest).get();
	}

	@Test
	public void testVerifyWorksForVerkeyCachedInWallet() throws Exception {
		String identityJson = String.format(IDENTITY_JSON_TEMPLATE, myDid, myVerkey);
		Signus.storeTheirDid(wallet, identityJson).get();

		Boolean valid = Signus.verifySignature(wallet, pool, myDid, MESSAGE, SIGNATURE).get();
		assertTrue(valid);
	}

	@Test
	public void testVerifyWorksForGetNymFromLedger() throws Exception {
		Boolean valid = Signus.verifySignature(wallet, pool, myDid, MESSAGE, SIGNATURE).get();
		assertTrue(valid);
	}

	@Test
	public void testVerifyWorksForOtherSigner() throws Exception {
		String identityJson = String.format(IDENTITY_JSON_TEMPLATE, trusteeDid, trusteeVerkey);
		Signus.storeTheirDid(wallet, identityJson).get();

		identityJson = String.format(IDENTITY_JSON_TEMPLATE, myDid, myVerkey);
		Signus.storeTheirDid(wallet, identityJson).get();

		byte[] signature = Signus.sign(wallet, trusteeDid, MESSAGE).get();
		Boolean valid = Signus.verifySignature(wallet, pool, myDid, MESSAGE, signature).get();
		assertFalse(valid);
	}
}
