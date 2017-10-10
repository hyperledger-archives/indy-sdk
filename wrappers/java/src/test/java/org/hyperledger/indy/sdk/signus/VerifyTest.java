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
	private byte[] signature = {- 87, - 41, 8, - 31, 7, 107, 110, 9, - 63, - 94, - 54, - 42, - 94, 66, - 18, - 45, 63, - 47, 12, - 60, 8, - 45, 55, 27, 120, 94, - 52, - 109, 53, 104,
			103, 61, 60, - 7, - 19, 127, 103, 46, - 36, - 33, 10, 95, 75, 53, - 11, - 46, - 15, - 105, - 65, 41, 48, 30, 9, 16, 78, - 4, - 99, - 50, - 46, - 111, 125, - 123, 109, 11};

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
		String identityJson = String.format("{\"did\":\"%s\",\"verkey\":\"%s\"}", myDid, myVerkey);
		Signus.storeTheirDid(wallet, identityJson).get();

		Boolean valid = Signus.verifySignature(wallet, pool, myDid, MESSAGE, signature).get();
		assertTrue(valid);
	}

	@Test
	public void testVerifyWorksForGetVerkeyFromLedger() throws Exception {
		Signus.storeTheirDid(wallet, String.format("{\"did\":\"%s\"}", myDid)).get();

		Boolean valid = Signus.verifySignature(wallet, pool, myDid, MESSAGE, signature).get();
		assertTrue(valid);
	}

	@Test
	public void testVerifyWorksForGetNymFromLedger() throws Exception {
		Boolean valid = Signus.verifySignature(wallet, pool, myDid, MESSAGE, signature).get();
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
