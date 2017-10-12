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

public class EncryptTest extends IndyIntegrationTestWithPoolAndSingleWallet {

	private String trusteeDid;
	private String trusteeVerkey;
	private String did;
	private String verkey;

	@Before
	public void before() throws Exception {
		CreateAndStoreMyDidResult result = Signus.createAndStoreMyDid(wallet, TRUSTEE_IDENTITY_JSON).get();
		trusteeDid = result.getDid();
		trusteeVerkey = result.getVerkey();

		CreateAndStoreMyDidResult nym = Signus.createAndStoreMyDid(wallet, MY1_IDENTITY_JSON).get();
		did = nym.getDid();
		verkey = nym.getVerkey();

		String nymRequest = Ledger.buildNymRequest(trusteeDid, did, verkey, null, null).get();
		Ledger.signAndSubmitRequest(pool, wallet, trusteeDid, nymRequest).get();
	}

	@Test
	public void testEncryptWorksForPkCachedInWallet() throws Exception {
		String identityJson = String.format(IDENTITY_JSON_TEMPLATE, did, verkey);
		Signus.storeTheirDid(wallet, identityJson).get();

		SignusResults.EncryptResult encryptResult = Signus.encrypt(wallet, pool, trusteeDid, did, MESSAGE).get();
		assertNotNull(encryptResult);
	}

	@Test
	public void testEncryptWorksForGetPkFromLedger() throws Exception {
		String identityJson = String.format("{\"did\":\"%s\"}", did);
		Signus.storeTheirDid(wallet, identityJson).get();

		SignusResults.EncryptResult encryptResult = Signus.encrypt(wallet, pool, trusteeDid, did, MESSAGE).get();
		assertNotNull(encryptResult);
	}

	@Test
	public void testEncryptWorksForGetNymFromLedger() throws Exception {
		SignusResults.EncryptResult encryptResult = Signus.encrypt(wallet, pool, trusteeDid, did, MESSAGE).get();
		assertNotNull(encryptResult);
	}

	@Test
	public void testEncryptWorksForUnknownMyDid() throws Exception {
		thrown.expect(ExecutionException.class);
		thrown.expectCause(new ErrorCodeMatcher(ErrorCode.WalletNotFoundError));

		String identityJson = String.format(IDENTITY_JSON_TEMPLATE, trusteeDid, trusteeVerkey);
		Signus.storeTheirDid(wallet, identityJson).get();

		Signus.encrypt(wallet, pool, DID1, trusteeDid, MESSAGE).get();
	}

	@Test
	public void testEncryptWorksForNotFoundNym() throws Exception {
		thrown.expect(ExecutionException.class);
		thrown.expectCause(new ErrorCodeMatcher(ErrorCode.CommonInvalidState));

		Signus.encrypt(wallet, pool, trusteeDid, DID1, MESSAGE).get();
	}
}
