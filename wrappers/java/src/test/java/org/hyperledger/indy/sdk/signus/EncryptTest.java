package org.hyperledger.indy.sdk.signus;

import org.hyperledger.indy.sdk.IndyIntegrationTestWithPoolAndSingleWallet;
import org.hyperledger.indy.sdk.InvalidStateException;
import org.hyperledger.indy.sdk.signus.SignusResults.CreateAndStoreMyDidResult;
import org.hyperledger.indy.sdk.wallet.WalletValueNotFoundException;
import org.junit.Before;
import org.junit.Test;

import java.util.concurrent.ExecutionException;

import static org.hamcrest.CoreMatchers.isA;
import static org.junit.Assert.assertNotNull;

public class EncryptTest extends IndyIntegrationTestWithPoolAndSingleWallet {

	private String did;

	@Before
	public void before() throws Exception {
		CreateAndStoreMyDidResult nym = Signus.createAndStoreMyDid(wallet, MY1_IDENTITY_JSON).get();
		did = nym.getDid();
	}

	@Test
	public void testEncryptWorksForPkCachedInWallet() throws Exception {
		String identityJson = String.format(IDENTITY_JSON_TEMPLATE, DID_TRUSTEE, VERKEY_TRUSTEE);
		Signus.storeTheirDid(wallet, identityJson).get();

		SignusResults.EncryptResult encryptResult = Signus.encrypt(wallet, pool, did, DID_TRUSTEE, MESSAGE).get();
		assertNotNull(encryptResult);
	}

	@Test
	public void testEncryptWorksForGetNymFromLedger() throws Exception {
		SignusResults.EncryptResult encryptResult = Signus.encrypt(wallet, pool, did, DID_TRUSTEE, MESSAGE).get();
		assertNotNull(encryptResult);
	}

	@Test
	public void testEncryptWorksForUnknownMyDid() throws Exception {
		thrown.expect(ExecutionException.class);
		thrown.expectCause(isA(WalletValueNotFoundException.class));

		String identityJson = String.format(IDENTITY_JSON_TEMPLATE, DID_TRUSTEE, VERKEY_TRUSTEE);
		Signus.storeTheirDid(wallet, identityJson).get();

		Signus.encrypt(wallet, pool, DID, DID_TRUSTEE, MESSAGE).get();
	}

	@Test
	public void testEncryptWorksForNotFoundNym() throws Exception {
		thrown.expect(ExecutionException.class);
		thrown.expectCause(isA(InvalidStateException.class));

		Signus.encrypt(wallet, pool, did, DID_MY2, MESSAGE).get();
	}
}
