package org.hyperladger.indy.sdk.pairwise;

import org.json.JSONArray;
import org.hyperledger.indy.sdk.IndyIntegrationTest;
import org.hyperledger.indy.sdk.pairwise.Pairwise;
import org.hyperledger.indy.sdk.signus.Signus;
import org.hyperledger.indy.sdk.signus.SignusResults.CreateAndStoreMyDidResult;
import org.hyperledger.indy.sdk.wallet.Wallet;
import org.junit.After;
import org.junit.Before;
import org.junit.Test;

import static org.junit.Assert.assertEquals;

public class ListPairwiseTest extends IndyIntegrationTest {

	private Wallet wallet;
	private String walletName = "pairwiseWallet";

	@Before
	public void createWallet() throws Exception {

		Wallet.createWallet("default", walletName, "default", null, null).get();
		this.wallet = Wallet.openWallet(walletName, null, null).get();
	}

	@After
	public void deleteWallet() throws Exception {
		this.wallet.closeWallet().get();
		Wallet.deleteWallet(walletName, null).get();
	}

	@Test
	public void testListPairwiseWorks() throws Exception {
		CreateAndStoreMyDidResult myDid = Signus.createAndStoreMyDid(this.wallet, "{}").get();
		CreateAndStoreMyDidResult theirDid = Signus.createAndStoreMyDid(this.wallet, "{}").get();

		Signus.storeTheirDid(this.wallet, String.format("{\"did\":\"%s\"}", theirDid.getDid())).get();

		Pairwise.createPairwise(wallet, theirDid.getDid(), myDid.getDid(), null).get();

		String listPairwise = Pairwise.listPairwise(wallet).get();
		JSONArray listPairwiseArray = new JSONArray(listPairwise);

		assertEquals(1, listPairwiseArray.length());
		assertEquals(listPairwiseArray.getString(0),
				String.format("{\"my_did\":\"%s\",\"their_did\":\"%s\"}", myDid.getDid(), theirDid.getDid()));
	}

	@Test
	public void testListPairwiseWorksForEmptyResult() throws Exception {
		String listPairwise = Pairwise.listPairwise(wallet).get();
		JSONArray listPairwiseArray = new JSONArray(listPairwise);
		assertEquals(0, listPairwiseArray.length());
	}
}
