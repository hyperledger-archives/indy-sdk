package org.hyperladger.indy.sdk.pairwise;

import org.hyperledger.indy.sdk.ErrorCode;
import org.hyperledger.indy.sdk.ErrorCodeMatcher;
import org.hyperledger.indy.sdk.IndyIntegrationTest;
import org.hyperledger.indy.sdk.pairwise.Pairwise;
import org.hyperledger.indy.sdk.signus.Signus;
import org.hyperledger.indy.sdk.signus.SignusResults.CreateAndStoreMyDidResult;
import org.hyperledger.indy.sdk.wallet.Wallet;
import org.json.JSONObject;
import org.junit.After;
import org.junit.Before;
import org.junit.Test;

import java.util.concurrent.ExecutionException;

import static org.junit.Assert.assertEquals;

public class GetPairwiseTest extends IndyIntegrationTest {

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
	public void testGetPairwiseWorks() throws Exception {
		String metadata = "some metadata";
		Pairwise.createPairwise(wallet, theirDid, myDid, metadata).get();

		String pairwiseInfoJson = Pairwise.getPairwise(wallet, theirDid).get();
		JSONObject pairwiseInfo = new JSONObject(pairwiseInfoJson);

		assertEquals(myDid, pairwiseInfo.getString("my_did"));
		assertEquals(metadata, pairwiseInfo.getString("metadata"));
	}

	@Test
	public void testGetPairwiseWorksForNotCreated() throws Exception {
		thrown.expect(ExecutionException.class);
		thrown.expectCause(new ErrorCodeMatcher(ErrorCode.WalletNotFoundError));

		Pairwise.getPairwise(wallet, theirDid).get();
	}
}
