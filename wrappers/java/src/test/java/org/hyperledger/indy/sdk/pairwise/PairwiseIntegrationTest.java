package org.hyperledger.indy.sdk.pairwise;


import org.hyperledger.indy.sdk.IndyIntegrationTestWithSingleWallet;
import org.hyperledger.indy.sdk.signus.Signus;
import org.hyperledger.indy.sdk.signus.SignusResults;
import org.junit.Before;

public class PairwiseIntegrationTest extends IndyIntegrationTestWithSingleWallet {

	protected String myDid;
	String theirDid;
	static final String metadata = "some metadata";
	static final String PAIR_TEMPLATE = "{\"my_did\":\"%s\",\"their_did\":\"%s\"}";


	@Before
	public void createDids() throws Exception {
		SignusResults.CreateAndStoreMyDidResult result = Signus.createAndStoreMyDid(wallet, "{}").get();
		myDid = result.getDid();

		result = Signus.createAndStoreMyDid(wallet, "{}").get();
		theirDid = result.getDid();

		Signus.storeTheirDid(wallet, String.format("{\"did\":\"%s\"}", theirDid)).get();
	}
}
