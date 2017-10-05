package org.hyperladger.indy.sdk.pairwise;

import org.hyperledger.indy.sdk.IndyIntegrationTest;
import org.hyperledger.indy.sdk.pairwise.Pairwise;
import org.hyperledger.indy.sdk.signus.Signus;
import org.hyperledger.indy.sdk.signus.SignusResults.CreateAndStoreMyDidResult;
import org.hyperledger.indy.sdk.wallet.Wallet;
import org.junit.After;
import org.junit.Before;
import org.junit.Test;

import static org.junit.Assert.assertFalse;
import static org.junit.Assert.assertTrue;

public class PairwiseExistsTest extends IndyIntegrationTest {

	private Wallet wallet;
	private String walletName = "pairwiseWallet";
	private String myDid;
	private String theirDid;

	@Before
	public void createWallet() throws Exception {

		Wallet.createWallet("default", walletName, "default", null, null).get();
		this.wallet = Wallet.openWallet(walletName, null, null).get();

		CreateAndStoreMyDidResult result = Signus.createAndStoreMyDid(this.wallet, "{}").get();
		myDid = result.getDid();

		result = Signus.createAndStoreMyDid(this.wallet, "{}").get();
		theirDid = result.getDid();

		Signus.storeTheirDid(this.wallet, String.format("{\"did\":\"%s\"}", theirDid)).get();
	}

	@After
	public void deleteWallet() throws Exception {
		this.wallet.closeWallet().get();
		Wallet.deleteWallet(walletName, null).get();
	}

	@Test
	public void testPairwiseExistsWorks() throws Exception {
		Pairwise.createPairwise(wallet, theirDid, myDid, null).get();
		assertTrue(Pairwise.isPairwiseExists(wallet, theirDid).get());
	}

	@Test
	public void testPairwiseExistsWorksForNotCreated() throws Exception {
		assertFalse(Pairwise.isPairwiseExists(wallet, theirDid).get());
	}
}
