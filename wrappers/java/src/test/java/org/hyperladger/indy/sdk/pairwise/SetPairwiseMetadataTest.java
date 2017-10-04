package org.hyperladger.indy.sdk.pairwise;

import org.hyperledger.indy.sdk.ErrorCode;
import org.hyperledger.indy.sdk.ErrorCodeMatcher;
import org.hyperledger.indy.sdk.IndyIntegrationTest;
import org.hyperledger.indy.sdk.pairwise.Pairwise;
import org.hyperledger.indy.sdk.signus.Signus;
import org.hyperledger.indy.sdk.signus.SignusResults.CreateAndStoreMyDidResult;
import org.hyperledger.indy.sdk.wallet.Wallet;
import org.junit.After;
import org.junit.Before;
import org.junit.Test;

import java.util.concurrent.ExecutionException;

import static org.junit.Assert.assertEquals;
import static org.junit.Assert.assertNotEquals;


public class SetPairwiseMetadataTest extends IndyIntegrationTest {

	private Wallet wallet;
	private String walletName = "pairwiseWallet";
	private String myDid;
	private String theirDid;
	private String metadata = "metadata";


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
	public void testSetPairwiseMetadataWorks() throws Exception {
		Pairwise.createPairwise(wallet, theirDid, myDid, null).get();
		String pairwiseWithoutMetadata = Pairwise.getPairwise(wallet, theirDid).get();

		Pairwise.setPairwiseMetadata(wallet, theirDid, metadata).get();
		String pairwiseWithMetadata = Pairwise.getPairwise(wallet, theirDid).get();

		assertNotEquals(pairwiseWithoutMetadata, pairwiseWithMetadata);
		assertEquals(String.format("{\"my_did\":\"%s\",\"metadata\":\"%s\"}", myDid, metadata), pairwiseWithMetadata);

	}

	@Test
	public void testSetPairwiseMetadataWorksForNotCreatedPairwise() throws Exception {
		thrown.expect(ExecutionException.class);
		thrown.expectCause(new ErrorCodeMatcher(ErrorCode.WalletNotFoundError));

		Pairwise.setPairwiseMetadata(wallet, theirDid, metadata).get();
	}
}
