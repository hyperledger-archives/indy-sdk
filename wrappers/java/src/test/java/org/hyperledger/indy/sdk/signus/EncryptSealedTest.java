package org.hyperledger.indy.sdk.signus;

import org.hyperledger.indy.sdk.ErrorCode;
import org.hyperledger.indy.sdk.ErrorCodeMatcher;
import org.hyperledger.indy.sdk.IndyIntegrationTestWithPoolAndSingleWallet;
import org.hyperledger.indy.sdk.ledger.Ledger;
import org.hyperledger.indy.sdk.signus.SignusResults.CreateAndStoreMyDidResult;
import org.junit.Before;
import org.junit.Test;

import java.util.concurrent.ExecutionException;

import static org.junit.Assert.assertNotNull;

public class EncryptSealedTest extends IndyIntegrationTestWithPoolAndSingleWallet {

	private String did;
	private String verkey;

	@Before
	public void before() throws Exception {
		CreateAndStoreMyDidResult result = Signus.createAndStoreMyDid(wallet, TRUSTEE_IDENTITY_JSON).get();
		String trusteeDid = result.getDid();

		CreateAndStoreMyDidResult nym = Signus.createAndStoreMyDid(wallet, MY1_IDENTITY_JSON).get();
		did = nym.getDid();
		verkey = nym.getVerkey();

		String nymRequest = Ledger.buildNymRequest(trusteeDid, did, verkey, null, null).get();
		Ledger.signAndSubmitRequest(pool, wallet, trusteeDid, nymRequest).get();
	}

	@Test
	public void testEncryptSealedWorksForPkCachedInWallet() throws Exception {
		String identityJson = String.format(IDENTITY_JSON_TEMPLATE, did, verkey);
		Signus.storeTheirDid(wallet, identityJson).get();

		byte[] encryptResult = Signus.encryptSealed(wallet, pool, did, MESSAGE).get();
		assertNotNull(encryptResult);
	}

	@Test
	public void testEncryptSealedWorksForGetPkFromLedger() throws Exception {
		String identityJson = String.format("{\"did\":\"%s\"}", did);
		Signus.storeTheirDid(wallet, identityJson).get();

		byte[] encryptResult = Signus.encryptSealed(wallet, pool, did, MESSAGE).get();
		assertNotNull(encryptResult);
	}

	@Test
	public void testEncryptSealedWorksForGetNymFromLedger() throws Exception {
		byte[] encryptResult = Signus.encryptSealed(wallet, pool, did, MESSAGE).get();
		assertNotNull(encryptResult);
	}

	@Test
	public void testEncryptSealedWorksForNotFoundNym() throws Exception {
		thrown.expect(ExecutionException.class);
		thrown.expectCause(new ErrorCodeMatcher(ErrorCode.CommonInvalidState));
		
		Signus.encryptSealed(wallet, pool, DID1, MESSAGE).get();
	}
}
